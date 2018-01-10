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
}
