use std::io;

fn main() {
    loop {
        println!("Please input a num:");

        let mut number = String::new();

        io::stdin().read_line(&mut number).expect(
            "Failed to read line.",
        );

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };

        println!("fibonacci({}) = {}", number, fibonacci(number));
    }
}

fn fibonacci(number: u32) -> u32 {
    match number {
        0 => 0,
        1 => 1,
        _ => fibonacci(number - 2) + fibonacci(number - 1),
    }
}