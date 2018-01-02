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
            &Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            &Coin::Nickel => 5,
            &Coin::Dime => 10,
            &Coin::Quarter => 25,
        }
    }
}

fn main() {
    let dime = Coin::Dime;
    let nickel = Coin::Nickel;
    let penny = Coin::Penny;
    let quarter = Coin::Quarter;

    println!("{}", penny.value());
    println!("{}", dime.value());
    println!("{}", nickel.value());
    println!("{}", quarter.value());
}
