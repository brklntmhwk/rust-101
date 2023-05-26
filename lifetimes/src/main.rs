#![allow(unused)]

/* The raison d'etre of lifetimes  */
// to prevent dangling references
// lifetimes define how long references should last and keep the relationships among multiple elements consistent

/* Lifetime annotations in struct definitions */
// can define structs to hold refs, but in that case a lifetime annotation is needed on every ref
// instances of ImportantExcerpt cannot live longer than the ref it holds in its part field
struct ImportantExcerpt<'a> {
    part: &'a str,
}

/* Lifetime annotations in method definitions */
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    // Rust allows variables to be initiated without an initial value, as long as they are not used
    // if done, you'll get a compile time error, which shows that Rust indeed does not allow null values
    // so at this moment, this is fine
    let r: &i32;
    {
        // error: `x` does not live long enough
        // once it gets out of the scope, x is no longer valid, whereas r is still
        let x: i32 = 5;
        r = &x;
    }
    // here occurs an error out of the scope, with the x value passed away
    // println!("r: {}", r);

    /* Lifetime annotation syntax */
    // basically, it's equalized to be the shortest of all lifetimes listed up
    // the generic lifetime 'a will get the concrete lifetime that is equal to the shorter of the lifetime candidates x or y

    let string1: String = String::from("abcd");
    let string2: &str = "xyz";
    let result: &str = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // it's customary to start with a when you use it (others also work fine though)
    // in this case, the return type &str doesn't clarify whether the ref is borrowed from a or b and throws a missing lifetime specifier error, so add it like 'a after &
    // to use it in func signatures, also have to add it with angle brackets
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        // the above tells that the returned ref will be valid as long as both the params are
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let novel: String = String::from("Call me Ishmael. Some years ago...");
    let first_sentence: &str = novel.split('.').next().expect("Could not find a '.'");
    let i: ImportantExcerpt = ImportantExcerpt {
        part: first_sentence,
    };

    /* The static lifetime */
    // with it, the subject is stored directly in the program's binary, being always available
    // all str literals have it, so this is like: &str ------> &'static str
    let s: &str = "I have a static lifetime.";

    /* Generic type params, trait bounds, and lifetimes all together! */
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

/* Rust has a borrow checker that compares scopes to determine whether all borrows are valid */
// fn main() {
//     let r;                // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {}", r); //          |
// }                         // ---------+

// fn main() {
//     let x = 5;            // ----------+-- 'b
//                           //           |
//     let r = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {}", r); //   |       |
//                           // --+       |
// }                         // ----------+

/* Three rules the Rust compiler follow to figure out the lifetimes of the refs when there are no explicit annotations indicated by coders  */
// 1. the compiler assigns a lifetime parameter to each parameter that’s a reference
// e.g. fn first_word(s: &str) -> &str { ------> fn first_word<'a>(s: &'a str) -> &str {

// 2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// e.g. fn first_word<'a>(s: &'a str) -> &str { ------> fn first_word<'a>(s: &'a str) -> &'a str {

// 3. if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters
// e.g. impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }
// ------>
// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&'a self, announcement: &'a str) -> &'a str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }
// if the compiler gets to the end of the above three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error.
