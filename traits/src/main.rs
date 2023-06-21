pub mod aggregator;

use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("mikeybkats"),
        content: String::from("hello dearies!"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
