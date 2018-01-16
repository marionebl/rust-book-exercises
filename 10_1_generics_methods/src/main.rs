extern crate num;

use num::{Num};

struct Point<T: Num> {
    x: T,
    y: T,
}

impl<T: Num> Point<T> {
    fn x(&self) -> &T where T: num::Num {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // E0277 the trait bound `&str: num::Num` is not satisfied
    // let p = Point { x: "foo", y: "bar" };
    // println!("p.x = {}", p.x());

    let p = Point { x: 1, y: 1 };
    println!("p.x = {}", p.x());

    // E0599 
    // p.distance_from_origin();

    let p = Point { x: 5.0, y: 10.0 };
    let d = p.distance_from_origin();

    println!("d = {}", d);
}
