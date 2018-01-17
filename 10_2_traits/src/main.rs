mod aggregator;

use aggregator::{Summarizable, NewsArticle, Tweet};

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
}
