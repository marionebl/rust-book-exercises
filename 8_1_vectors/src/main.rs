#[allow(unused_doc_comment)]

fn main() {
    let empty: Vec<i32> = Vec::new();

    /// No type annotation needed here
    let consecutive_three = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("empty: {:?}", empty);
    println!("consecutive_three {:?}", consecutive_three);
    println!("v {:?}", v);
}
