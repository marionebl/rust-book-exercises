use std::ops::Add;

fn main() {
    let mut _s = String::new();
    let _bar = "bar";
    _s.push_str(&_bar);
    println!("_s is {}, _bar is {}", _s, _bar);

    let data = "initial contents";
    let _t = data.to_string();
    let _u = "initial_contents".to_string();

    let _v = String::from(data);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2; // String::add takes ownership of the left hand of the addition

    // https://doc.rust-lang.org/error-index.html#E0382
    // println!("{}", s1);

    println!("s2: {}, s3: {}", s2, s3);

    let t1 = String::from("Hello, ");
    let t2 = String::from("world!");

    let t3 = t1.add(&t2);

    // https://doc.rust-lang.org/error-index.html#E0382
    // println!("{}", t1);

    println!("t2: {}, t3: {}", t2, t3);
}