use std::io;

fn main() {
    let number = get_input();

    let condition = true;

    let divisor = if number % 4 == 0 {
        4
    } else if number % 3 == 0 {
        3
    } else if number % 2 == 0 {
        2
    } else {
        0
    };

    if divisor > 0 {
        println!("{number} is divisible by {divisor}", number = number, divisor = divisor)
    } else {
        println!("{number} is not divisible by 4, 3, or 2", number = number);
    }
}

fn get_input() -> i32 {
    let mut input = String::new();

    loop {
        println!("Give a number:");
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => {
                break num;
            },
            Err(_) => {
                println!("input must be number");
                continue;
            }
        }
    }
}
