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
            match File::create(name) {
                Ok(file) => file,
                Err(e) => {
                    panic!("Tried to create {} but there was a problem: {:?}", name, e)
                }
            }
        },
        Err(error) => {
            panic!("Problem while opening {}: {:?}", name, error)
        }
    };

    match fs::remove_file(name) {
        Ok(_) => (),
        Err(error) => {
            panic!("Problem while cleaning up {}: {:?}", name, error)
        }
    }
}
