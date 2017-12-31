fn main() {
    let mut s = String::from("hello");
    change(&mut s);

    let len = calculate_length(&s); // create a ptr "s" to ptr "s1"
    // calculate_length uses a reference to s1, so
    // s1 stays valid after the scope of calculate_length was closed

    println!("The length of '{}' is {}.", s, len);

    try_race();
    try_corrupt();
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

fn try_race() {
    let mut s = String::from("Hello");

    {
        let r1 = &mut s;
        change(r1);
        println!("r1: {}", r1);
    }

    {
        let r2 = &mut s;
        change(r2);
        println!("r2: {}", r2);
    }

    // https://doc.rust-lang.org/error-index.html#E0499
    // let r2 = &mut s;
}

fn try_corrupt() {
    let mut s = String::from("hello");

    {
        let c1 = &s;
        let c2 = &s;
        println!("c1: {}, c2: {}", c1, c2);
    }

    {
        let c3 = &mut s;
        change(c3);
        println!("c3: {}", c3);
    }

    // Immutable (c1, c2) and mutable borrows may not share the same scope
    // let c1 = &s;
    // let c2 = &s;
    // https://doc.rust-lang.org/error-index.html#E0596
    // let c3 = &mut s;
}

// https://doc.rust-lang.org/error-index.html#E0596
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s;
//     // s goes out of scope here, leaving &s dangling
// }