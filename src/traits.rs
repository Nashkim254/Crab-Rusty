use std::fmt::{format, Display};

pub struct Article {
    pub author: String,
    pub headline: String,
    pub content: String,
}
pub struct Tweet {
    pub username: String,
    pub reply: String,
    pub content: String,
    pub retweet: String,
}
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}:{}", self.headline, self.author)
    }
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub fn run() {
    let article = Article {
        author: String::from("JK. Rowling"),
        headline: String::from("Harry Potter"),
        content: String::from("Hogwarts school of witchcraft and soccery"),
    };

    let tweet = Tweet {
        username: String::from("Nash"),
        reply: String::from("JK. Rowling rocks"),
        content: String::from("Awesome read"),
        retweet: String::from("Absolutely!"),
    };

    println!("Tweet Summary: {}", tweet.summarize());
    println!("Article Summary: {}", article.summarize());
    notify(&article);
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news {}", item.summarize())
}
// impl<T: Display> ToString for T {}
