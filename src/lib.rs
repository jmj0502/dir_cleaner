//! # dir_cleaner
//! A simple `binary crate` written in order to learn how to create,
//! organize and distribute `Rust` apps.
//!
//! The main purpose of the crate is to help you identify duplicate files
//! in nested directories and simplify their deletion (if necessary).
//!
//! The crate is super simple to use, just call it from the
//! command line and provide the `relative path` of the folder
//! you want to inspect.
//!
//! ```rust,ignore
//! dir_cleaner ./test
//! ```
//! Once you've done so, the program will ask you for a `file name`;
//! such a name will be used to find any file with that __exact
//! name__ on the specified `directory` and its `respective subdirectories`.
//! Then, the program will proceed to expose the list of the
//! found files (along with their `creation_date` and their `relative path`) and ask you if you want to keep all of them,
//! if that's not the case it will help you with the deletion process.

use chrono::{DateTime, Utc};
use std::{fs, fmt};
use std::ops::Not;
use std::error::Error;
use std::fs::Metadata;
use std::path::PathBuf;
use std::io;
use std::fmt::Formatter;

struct DirInfo {
    metadata: Metadata,
    path_buf: PathBuf,
}

#[derive(Debug,Clone)]
struct ArgsError;

impl Error for ArgsError {}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Insufficient arguments provided.\n Usage: `low_level_four |relative_path_to_folder|`")
    }
}

/// Proceeds to gather the `files` located on the directory represented by the `path` specified through `args`,
/// as long as they match the `name` provided by the user when prompted to do. Once the information
/// of the different `files` is collected, the user will have the option to `keep` or `delete` all of them.
pub fn run(mut args: impl Iterator<Item=String>) -> Result<(), Box<dyn Error>> {
    args.next();

    let directory = match args.next() {
        Some(str) => str,
        None => return Err(ArgsError.into()),
    };

    let input = get_input(
        "Please, provide the name of the file you want to search (including its file extension)",
    );

    let mut files_info = get_dir_files(&directory, input.trim())?;
    for (i, file) in files_info.iter().enumerate() {
        println!("Entry {}", &i + 1);
        file.show_info();
    }

    let answer = get_input("Do you want to keep every file? \n(y/n)");
    if answer.trim().eq("y") {
        println!("Good Bye!");
        return Ok(());
    }

    loop {
        let answer = get_input(
            "Please provide the number associated to the file you want to delete.\nWrite done to quit",
        );
        let cleaned_answer = answer.trim();
        if cleaned_answer.eq("done") {
            println!("Good Bye!");
            break;
        }
        let index = cleaned_answer.parse::<usize>();
        let index = match index {
            Ok(index) => index,
            Err(_) => {
                println!("Invalid number provided.");
                continue;
            }
        };
        //let file = &files_info[&index - 1];
        if (index >= *&files_info.len() + 1) || (index <= 0) {
            println!("Please provide one of the listed numbers!");
            break;
        }
        let file = &files_info.swap_remove(&index - 1);
        file.delete()?;
        println!("File deleted!");
    }
    Ok(())
}

/// Stores relevant information (and some `metadata`) associated with a specific file,
/// in order to simply its manipulation at `fs-level` (E.G: Access, Deletion, Modification).
/// ## Example
/// ```
/// # use dir_cleaner::{File};
/// let name=  "test.txt";
/// let folder=  ".";
/// let creation_date=  "2022-07-23 12:33:01";
/// let path=  "./test.txt";
/// let file = File::new(name, folder, creation_date, path);
/// assert_eq!(&file.name, name);
/// assert_eq!(&file.folder, folder);
/// assert_eq!(&file.creation_date, creation_date);
/// ```
#[derive(PartialEq, Debug)]
pub struct File {
    pub name: String,
    pub folder: String,
    pub creation_date: String,
    path: String,
}

impl File {

    /// Generates a new instance of `File` using the data provided through its parameters.
    /// ## Examples:
    /// ```
    /// # use dir_cleaner::{File};
    /// let name=  "test.txt";
    /// let folder=  ".";
    /// let creation_date=  "2022-07-23 12:33:01";
    /// let path=  "./test.txt";
    /// let file = File::new(name, folder, creation_date, path);
    /// ```
    pub fn new(name: &str, folder: &str, creation_date: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            folder: folder.to_string(),
            creation_date: creation_date.to_string(),
            path: path.to_string(),
        }
    }

    /// Prints the `name`, `folder` and `creation_date` of a `File` using `\t` and `\n` chars,
    /// in order to meet a format equivalent to one level of `indentation`.
    /// ## Examples
    /// ```
    /// # use dir_cleaner::{File};
    /// let name=  "test.txt";
    /// let folder=  ".";
    /// let creation_date=  "2022-07-23 12:33:01";
    /// let path=  "./test.txt";
    ///
    /// let file = File::new(name, folder, creation_date, path);
    /// file.show_info();
    /// // Prints:
    /// //  test.txt
    /// //  current_folder
    /// //  2022-07-23 12:33:01
    /// ```
    pub fn show_info(&self) {
        println!(
            "\tfile name: {} \n\tdirectory: {} \n\tcreation date: {}",
            &self.name, &self.folder, &self.creation_date
        );
    }

    /// Removes the file located in the `path` contained on the `File` instance that called the method.
    /// ## Examples
    /// ```
    /// # use dir_cleaner::{File};
    /// # use chrono::{DateTime, Utc};
    /// # fn main() -> Result<(), std::io::Error> {
    /// #   std::fs::File::create("./test.txt").unwrap();
    ///     let f = std::fs::File::open("./test.txt").unwrap();
    ///     let name = "test.txt";
    ///     let path = "./test.txt";
    ///     let metadata = &f.metadata().unwrap();
    ///     let creation_date = metadata.created().unwrap();
    ///     let creation_date: DateTime<Utc> = creation_date.clone().into();
    ///     let creation_date = creation_date.format("%Y-%m-%d %H:%M:%S").to_string();
    ///     let file = File::new(name, ".", &creation_date, path);
    ///     file.delete()
    /// # }
    /// ```
    pub fn delete(&self) -> Result<(), std::io::Error> {
        fs::remove_file(&self.path)?;
        Ok(())
    }
}

/// Recursively traverses the directory located in the provided `path` and its respective subdirectories, in order
/// to gather the information of the files that have the provided `file_name` and collect it into a
/// `Vec` of `Files`.
/// ## Examples
/// ```
/// # use dir_cleaner::get_dir_files;
/// # std::fs::File::create("./test.txt").unwrap();
/// let mut files = match get_dir_files("./", "test.txt") {
///     Ok(f) => f,
///     Err(e) => {
///         eprintln!("{}", e);
///         std::process::exit(1);
///     },
/// };
/// assert_eq!(files.len(), 1);
/// assert_eq!(files[0].name, "test.txt");
/// # files[0].delete();
/// # files.remove(0);
/// ```
pub fn get_dir_files(path: &str, file_name: &str) -> Result<Vec<File>, std::io::Error> {
    let dir_entry = fs::read_dir(&path)?;
    let mut sub_dirs: Vec<String> = vec![];
    let mut files: Vec<File> = dir_entry.filter(|f| f.is_ok())
        .flatten()
        .map(|d| {
            let path = &d.path();
            let metadata = &d.metadata().unwrap();
            DirInfo {
                path_buf: path.to_owned(),
                metadata: metadata.to_owned(),
            }
        })
        .filter(|fi| {
            if fi.path_buf.is_file().not() {
                sub_dirs.push(fi.path_buf.to_str().unwrap().to_string());
            }
            fi.path_buf.is_file()
        })
        .map(|fi| {
            let file_path = &fi.path_buf.to_str().unwrap();
            let entry_name = &fi.path_buf.file_name().unwrap();
            let entry_name = entry_name.to_str().unwrap();
            let creation_date = fi.metadata.created().unwrap();
            let creation_date: DateTime<Utc> = creation_date.clone().into();
            let creation_date = creation_date.format("%Y-%m-%d %H:%M:%S").to_string();
            File::new(entry_name, &path, &creation_date, file_path)
        })
        .filter(|f| f.name.eq(file_name))
        .collect();

    for sub_dir in sub_dirs {
        let mut sub_files = get_dir_files(&sub_dir, file_name)?;
        files.append(&mut sub_files);
    }

    Ok(files)
}

/// Prints the provided `message` to `stdout` and proceeds to get user `input`.
/// ## Examples
/// ```
/// # use dir_cleaner::get_input;
/// let input = get_input("Please, provide a number!");
/// // Prints: "Please provide a number" and gets the input provided by the user.
/// ```
pub fn get_input(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read the provided information.");
    input
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn file_constructor() {
        // Arrange
        let name=  "test.txt";
        let folder=  ".";
        let creation_date=  "2022-07-23 12:33:01";
        let path=  "./test.txt";

        let file = File {
            name: name.to_owned(),
            folder: folder.to_owned(),
            creation_date: creation_date.to_owned(),
            path: path.to_owned(),
        };

        // Act and Assert
        assert_eq!(file, File::new(name, folder, creation_date, path));
    }

    #[test]
    fn file_delete() -> Result<(), std::io::Error> {
        //  Arrange
        let f = std::fs::File::create("./test.txt").unwrap();
        let name = "test.txt";
        let path = "./test.txt";
        let metadata = &f.metadata().unwrap();
        let creation_date = metadata.created().unwrap();
        let creation_date: DateTime<Utc> = creation_date.clone().into();
        let creation_date = creation_date.format("%Y-%m-%d %H:%M:%S").to_string();
        let file = File::new(name, ".", &creation_date, path);

        // Act
        let result = file.delete();

        // Assert (Should pass if the Ok variant is returned).
        result
    }

    #[test]
    fn get_dir_files() -> Result<(), std::io::Error> {
        // Arrange
        let f = std::fs::File::create("./text.txt").unwrap();
        let name = "text.txt";
        let path = std::path::Path::new("./").join(name);
        let path = path.to_str().unwrap();
        let metadata = &f.metadata().unwrap();
        let creation_date = metadata.created().unwrap();
        let creation_date: DateTime<Utc> = creation_date.clone().into();
        let creation_date = creation_date.format("%Y-%m-%d %H:%M:%S").to_string();
        let expected_file = File::new(name, "./", &creation_date, &path);

        // Act
        let files = super::get_dir_files("./", "text.txt").unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });

        // Assert
        assert_eq!(vec![expected_file], files);

        //teardown.
        files[0].delete()
    }
}
