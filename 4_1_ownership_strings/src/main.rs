fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s);

    let x = 5;
    let y = x; // Copy on stack
    // x stays valid

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // Copy ptr, len and cap on stack, move heap data
    // s1 now is invalid

    // Creates compiler error E0382
    // https://doc.rust-lang.org/error-index.html#E0382
    // println!("{}, world", s1);

    let s3 = s2.clone(); // Copy heap data of s2 into s3
    println!("s2 = {}, s3 = {}", s2, s3);

    takes_ownership(s2); // Move s2 to some_string param
    // s2 now is invalid

    borrows(&s3); // borrow s3
    // s3 stays valid

    takes_ownership(s3);
    // s3 now is invalid

    // Creates compiler error E0382
    // https://doc.rust-lang.org/error-index.html#E0382
    // takes_ownership(s2);

    makes_copy(y); // Copied on stack
    // y stays valid
    makes_copy(y);

    let s4 = gives_ownership();
    let s5 = String::from("hello");
    let s6 = gives_back(s5);

    takes_ownership(s6);
    takes_ownership(s4);
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_ownership(some_string: String) {
    println!("owned: {}", some_string);
}

fn gives_back(some_string: String) -> String {
    println!("give back: {}", some_string);
    some_string
}

fn borrows(some_string: &String) {
    println!("borrowed: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("copied: {}", some_integer);
}
