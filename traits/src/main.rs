pub mod aggregator;

use aggregator::{notify, returns_summarizable, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("mikeybkats"),
        content: String::from("hello dearies!"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    notify(&tweet);

    let new_tweet = returns_summarizable();

    println!("Another new tweet: {}", new_tweet.summarize());
}
