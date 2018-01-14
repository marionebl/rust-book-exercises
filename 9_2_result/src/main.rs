use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;

fn main() {
    let name = "hello.txt";

    let _f = open_or_create(name);
    fs::remove_file(name).expect("Failed cleaning up hello.txt");
}

fn open_or_create(filename: &str) -> Result<File, io::Error> {
    match File::open(filename) {
        Ok(file) => Ok(file),
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            File::create(filename)
        },
        Err(error) => Err(error)
    }
}