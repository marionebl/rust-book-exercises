#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn fits(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("The area of {rect:?} is {area} square pixels", rect=rect, area=rect.area());
    println!("rect fits rect2 {}", rect.fits(&rect2));
    println!("rect fits rect3 {}", rect.fits(&rect3));
}