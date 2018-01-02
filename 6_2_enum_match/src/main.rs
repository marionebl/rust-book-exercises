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

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}, none: {:?}", six, none);

    print_when_is_three(&Some(3));
    print_when_is_three(&Some(1));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn print_when_is_three(x: &Option<u8>) {
    if let &Some(3) = x {
        println!("three");
    }
}