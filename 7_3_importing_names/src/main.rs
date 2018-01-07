pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("Nested all the way");
            }
        }
    }
}

use a::series::of;
use a::series::of::nested_modules;

fn main() {
    of::nested_modules();
    nested_modules();
}
