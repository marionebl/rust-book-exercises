fn main() {
    let mut s = String::from("Hello world");
    let word = first_word_pos(&s);

    let lit = "hello world";
    let first = first_word(lit);
    println!("first: {}", first);

    {
        let hello = first_word(&s);
        println!("second: {}", hello);
    }

    {
        let hello = &s[0..word];
        println!("third: {}", hello);
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

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}