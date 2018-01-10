use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores_two: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("scores_two: {:?}", scores_two);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, field_value);

    println!("map: {:?}", map);

    // https://doc.rust-lang.org/error-index.html#E0382
    // println!("field_name: {}, field_value: {}", field_name, field_value);

    let score = map.get(&field_name);
    println!("score: {:?}", score);

    for (key, value) in &scores_two {
        println!("{}: {}", key, value);
    }

    // Overwrite scores.get("Blue")
    scores.insert(String::from("Blue"), 25);

    println!("overwritten scores: {:?}", scores);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(100);
    scores.entry(String::from("Green")).or_insert(25);

    println!("inserted scores: {:?}", scores);
}
