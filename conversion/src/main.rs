use std::convert::From;
use std::string::ToString;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Number { value: i32 }

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[derive(Debug)]
struct Circle { radius: i32 }

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let radius = s.parse::<i32>()?;

        Ok(Circle { radius })
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println! {"Sum: {}", sum};

    println!("{:?}", Circle::from_str("5").unwrap());
    println!("{:?}", "5".parse::<Circle>().unwrap());
}
