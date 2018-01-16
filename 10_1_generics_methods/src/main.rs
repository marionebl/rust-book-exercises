extern crate num;

use num::{Num};

struct Point<T: Num, U: Num> {
    x: T,
    y: U,
}

impl<T: Num, U: Num> Point<T, U> {
    fn x(&self) -> &T where T: Num, U: Num {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T: Num, U: Num> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W>
        where W: Num, V: Num {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // E0277 the trait bound `&str: num::Num` is not satisfied
    // let p = Point { x: "foo", y: "bar" };
    // println!("p.x = {}", p.x());

    let p1 = Point { x: 1, y: 1 };
    println!("p.x = {}", p1.x());

    // E0599 
    // p.distance_from_origin();

    let p2 = Point { x: 5.0, y: 10.0 };
    let d = p2.distance_from_origin();

    println!("d = {}", d);

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
