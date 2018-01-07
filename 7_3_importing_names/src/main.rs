pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("Nested all the way");
            }
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum MinimalTrafficLight {
    Red,
    Green
}

use a::series::of;
use a::series::of::nested_modules;
use TrafficLight::{Red, Yellow};
use MinimalTrafficLight::*; // This shadows 'Red' from TrafficLight!

fn main() {
    of::nested_modules();
    nested_modules();

    let _red = Red; // References MinimalTrafficLight::Red
    let _yellow = Yellow;
    let _green = TrafficLight::Green;

    let _minimal_green = Green;
    let _minimal_red = Red;
}
