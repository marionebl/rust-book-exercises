fn main() {
    let x = five(); // statement

    let y = { // blocks are expressions
        let x = 3;
        add_one(x)
    };

    println!("The value of y is {}", y); // expect 4
}

fn five() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    x + 1
}