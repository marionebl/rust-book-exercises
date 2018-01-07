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

    let third = &v[2];
    let third_get = v.get(2);

    println!("third: {}", third);
    println!("third_get: {:?}", third_get);

    // !panic
    // let non_existant = &v[999]; 
    let non_existant_get = v.get(1000);

    println!("non_existant_get: {:?}", non_existant_get);

    invalid_references();
}

fn invalid_references() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // https://doc.rust-lang.org/error-index.html#E0502
    // let first = &v[0];

    {
        // solution: descope before mutation
        let scoped =  &v[0];
        println!("invalid_references scoped {}", scoped);
    }

    v.push(9);
}