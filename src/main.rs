use std::env;
use std::process;

fn main() {
    let args = env::args();

    dir_cleaner::run(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}
