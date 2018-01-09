fn main() {
    let mut _s = String::new();
    let _bar = "bar";
    _s.push_str(&_bar);
    println!("_s is {}, _bar is {}", _s, _bar);

    let data = "initial contents";
    let _t = data.to_string();
    let _u = "initial_contents".to_string();

    let _v = String::from(data);
}