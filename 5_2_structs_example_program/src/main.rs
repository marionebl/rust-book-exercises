struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    println!("The area is {} square pixels", area(&rect));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}