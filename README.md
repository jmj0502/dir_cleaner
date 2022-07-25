# dir_cleaner
A simple `binary crate` written in order to learn how to create,
organize and distribute `Rust` apps.

The main purpose of the crate is to help you identify duplicate files
in nested directories and simplify their deletion (if necessary).

The crate is super simple to use, just call it from the
command line and provide the `relative path` of the folder
you want to inspect. EG:
```
dir_cleaner ./test
```

Once you've done so, the program will ask you for a `file name`;
such a name will be used to find any file with that __exact
name__ on the specified `directory` and its `respective subdirectories`.
Then, the program will proceed to expose the list of the
found files (along with their `creation_date` and their `relative path`) and ask you if you want to keep all of them,
if that's not the case it will help you with the deletion process.