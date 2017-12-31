fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // create a ptr "s" to ptr "s1"
    // calculate_length uses a reference to s1, so
    // s1 stays valid after the scope of calculate_length was closed

    println!("The length fof '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}