fn main() {
    let mut s = String::from("Hello world");
    let word = first_word(&s);

    {
        let hello = &s[0..word];
        println!("{}", hello);
    }

    s.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}