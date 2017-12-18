const Y: u32 = 10;

fn main() {
    let x = 5;
    println!("The value of x is: {}, y is {}", x, Y);

    // Reassignment shadows previous
    // assignment
    let x = x + 1;
    println!("The value of x is: {}, y is {}", x, Y);

    // Reassignment allows type changes
    // Would produce compiler errors: 
    // let mut x = 5;
    // x = x.to_string()
    let x = x.to_string();
    println!("The value of x is: {}, y is {}", x, Y);
}
