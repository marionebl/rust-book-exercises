mod aggregator;

use aggregator::{Summarizable, NewsArticle, Tweet, WeatherForecast};

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.", self.high_temp, self.low_temp,
        self.chance_of_precipitation)
    }
}

struct CustomContent {
    content: String,
}

impl Summarizable for CustomContent {}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

    let article = NewsArticle {
        headline: String::from("Headline"),
        location: String::from("NY"),
        author: String::from("Jon Doe"),
        content: String::from("content")
    };

    println!("1 article: {}", article.summary());

    let cast = WeatherForecast {
        high_temp: 10.0,
        low_temp: 0.0,
        chance_of_precipitation: 64.0
    };

    println!("weather forecast: {}", cast.summary());

    let custom = CustomContent {
        content: String::from("content")
    };

    println!("custom content: {}", custom.summary());
}
