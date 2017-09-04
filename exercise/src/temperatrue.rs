use std::io;

fn main() {
    loop {
        println!("1 - Celsius\n2 - Fahrenheit");

        let mut option = String::new();

        io::stdin().read_line(&mut option).expect(
            "Failed to read line.",
        );

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };

        println!("Please input the temperatrue:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect(
            "Failed to read line.",
        );

        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };

        transform(option, input);
    }

    fn transform(option: u32, temperatrue: f64) {
        if option == 1 {
            println!("{}℃ = {}℉", temperatrue, temperatrue * 1.8 + 32f64);
        } else if option == 2 {
            println!("{}℉ = {}℃", temperatrue, (temperatrue - 32f64) / 1.8);
        } else {
            println!("Invalid option.");
        }
    }
}