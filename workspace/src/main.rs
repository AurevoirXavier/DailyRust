extern crate add_one;
extern crate add_two;

fn main() {
    let num = 10;

    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
    println!("Hello, world! {} plus one is {}!", num, add_two::add_two(num));
}
