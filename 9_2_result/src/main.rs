use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    // Equivalent to let _f = f.unwrap();
    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
}
