use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let filename = "hello.txt";
    let username = read_username_from_file(filename)
        .expect("Could not read username");

    println!("username: {}", username);

    fs::remove_file(filename).expect("Failed cleaning up hello.txt");
}

fn open_or_create(filename: &str) -> Result<File, io::Error> {
    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)
}

/**
 * let x = fn(y)? is equivalent to
 * 
 * let x = match fn(y) {
 *   Ok(z) => z, // unwrap result
 *   Err(e) => return Err(e) // propagates error to parent scope
 * }
 */
fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    
    open_or_create(filename)?.read_to_string(&mut s)?;

    Ok(s)
}