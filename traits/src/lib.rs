#![allow(unused)]

/* Traits */
// more cross-sectional with types being longitudinal?
// defines functionality that a particular type has and can share with other types

// Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior.
// unlike other dynamically typed languages, Rust does type checks at compile time, so you'll have to fix the problems before runtime

/* Defining a trait */
// Summary is a functionality that's applicable to types that summarize something (both NewsArticle and Tweet might do that!)
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        /* Default implementations */
        // this is a default behavior and can be overridden from anywhere it's implemented
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/* Implementing a trait on a type */
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

/* Traits as parameters */
// the param accepts any type that implements the specified trait
pub fn notify(item: &impl Summary) {
    // in the func, whatever defined in the trait is available
    println!("Breaking news!: {}", item.summarize());
}
// The above is actually sugar syntax of the below
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// when params are multiple and they take different types, using "impl Trait" is appropriate
// e.g. pub fn notify(item1: &impl Summary, item2: &impl Summary) {
// but if they are supposed to take the same type, then it should be like this
// e.g. pub fn notify<T: Summary>(item1: &T, item2: &T) {
// because the generic type T constrains the func such that the concrete type of the value passed as an argument for item1 and item2 must be the same.

use std::fmt::{Debug, Display};
// can specify multiple trait bounds with the + syntax
pub fn notify_multiple_trait_bounds(item: &(impl Summary + Display)) {
    // in the func, whatever defined in the trait is available
    println!("Breaking news!: {}", item.summarize());
}
// the generic type T ver. is like this
// pub fn notify<T: Summary + Display>(item: &T) { ... }

// clearer trait bounds with where clauses when they get stacked enormously
fn some_func<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1 + 1
}
// without a where clause, the above looks like this
// fn some_func<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// conditional implement methods with trait bounds
struct Pair<T> {
    x: T,
    y: T,
}

// this is implemented with no condition
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// this is implemented only if the specified type has both Display and PartialOrd traits
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
