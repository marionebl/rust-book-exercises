struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // E0599 
    // p.distance_from_origin();

    let p = Point { x: 5.0, y: 10.0 };
    let d = p.distance_from_origin();

    println!("d = {}", d);
}
