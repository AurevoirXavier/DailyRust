extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

#[derive(Debug)]
struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value: value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect(
            "Failed to read line.",
        );

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };

        let guess = Guess::new(guess);

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too Big."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
