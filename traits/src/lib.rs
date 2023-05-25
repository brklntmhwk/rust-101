/* Traits */
// more cross-sectional with types being longitudinal?
// defines functionality that a particular type has and can share with other types

// Summary is a functionality that's applicable to types that summarize something (both NewsArticle and Tweet might do that!)
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        // this is a default behavior and can be overridden every time it's implemented
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// use aggregator::{Summary, Tweet};

// fn main() {}
