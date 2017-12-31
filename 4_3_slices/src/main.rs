fn main() {
    let mut s = String::from("Hello world");
    let word = first_word_pos(&s);

    {
        let hello = first_word(&s);
        println!("first: {}", hello);
    }

    {
        let hello = &s[0..word];
        println!("second: {}", hello);
    }

    s.clear();
}

fn first_word_pos(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}