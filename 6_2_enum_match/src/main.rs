#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

impl Coin {
    fn value(&self) -> i32 {
        match self {
            &Coin::Penny => 1,
            &Coin::Nickel => 5,
            &Coin::Dime => 10,
            &Coin::Quarter => 25,
        }
    }
}

fn main() {
    let dime = Coin::Dime;
    let val = dime.value();

    println!("{}", val);
}