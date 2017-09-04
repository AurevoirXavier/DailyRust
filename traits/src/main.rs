extern crate traits;

use traits::Summarizable;
use traits::Summarizable2;

struct WeatherForecast {
    high_temp: f64,
    low_tamp: f64,
    chance_of_precipitation: f64
}

struct DefaultTrait {}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of precipitation is {}%.", self.high_temp, self.low_tamp, self.chance_of_precipitation)
    }
}

impl Summarizable for DefaultTrait {}



fn main() {
    let tweet = traits::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    let forecast = WeatherForecast {
        high_temp: 1.0,
        low_tamp: 0.5,
        chance_of_precipitation: 0.2
    };

    let default_trait = DefaultTrait {};

    println!("1 new tweet: {}", tweet.summary());

    println!("{}", forecast.summary());

    println!("{}", default_trait.summary());

    println!("1 new tweet: {}", tweet.summary2());
}