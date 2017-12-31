fn main() {
    let mut s = String::from("hello");
    change(&mut s);

    let len = calculate_length(&s); // create a ptr "s" to ptr "s1"
    // calculate_length uses a reference to s1, so
    // s1 stays valid after the scope of calculate_length was closed

    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}

// https://doc.rust-lang.org/error-index.html#E0596
// fn change(s: &String) {
//     s.push_str(", world");
// }

fn change(s: &mut String) {
    s.push_str(", world");
}