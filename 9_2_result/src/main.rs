use std::fs;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let name = "hello.txt";

    let _f = match File::open(name) {
        Ok(file) => file,
        // Thing(ref value) Match value, provide reference
        // Thing(&value) Match reference
        Err(ref error) if error.kind() == ErrorKind::NotFound => { // Match guard
            File::create(name).expect("Failed to create hello.txt")
        },
        Err(error) => {
            panic!("Problem while opening {}: {:?}", name, error)
        }
    };

    fs::remove_file(name).expect("Failed cleaning up hello.txt");
}
