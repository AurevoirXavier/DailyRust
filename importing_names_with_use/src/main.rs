pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use a::series::of;
// use a::series::of::nested_modules;

use TrafficLight::{Red, Yellow};
// use TrafficLight::*;

fn main() {
    of::nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
