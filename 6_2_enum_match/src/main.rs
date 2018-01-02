#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
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
            &Coin::Quarter(ref state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
}

fn main() {
    let dime = Coin::Dime;
    let nickel = Coin::Nickel;
    let penny = Coin::Penny;
    let alabama_quarter = Coin::Quarter(UsState::Alabama);
    let alaska_quarter = Coin::Quarter(UsState::Alaska);

    println!("{}", penny.value());
    println!("{}", dime.value());
    println!("{}", nickel.value());
    println!("{}", alabama_quarter.value());
    println!("{}", alaska_quarter.value());
}
