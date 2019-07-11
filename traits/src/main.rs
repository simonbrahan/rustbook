mod news_article;
mod summary;
mod tweet;

use news_article::NewsArticle;
use summary::Summary;
use tweet::Tweet;

fn digest(items: Vec<&Summary>) {
    for item in items.iter() {
        println!("{}", item.summarise());
    }
}

fn main() {
    let tweet = Tweet::new("skawid", "Owl noises");

    let news = NewsArticle {
        headline: String::from("Owls"),
        location: String::from("Woods"),
        author: String::from("skawid"),
        content: String::from("There are owls"),
    };

    let items: Vec<&Summary> = vec![&tweet, &news];

    digest(items);
}
