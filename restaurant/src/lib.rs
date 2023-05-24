#![allow(unused)]

/* when putting all mods together in the root file */
// // contains sections engaged in customer service (front)
// mod front_of_house {
//     // nested modules

//     // children mods can refer to their parents' elements but the reverse is not allowed by default
//     // so adding pub to every element that should be accessible from their parents' standpoint is necessary to make it possible
//     pub mod hosting {
//         // by the same token, add pub to whatever should be accessible
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }
//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }

// mod back_of_house {
//     // by the same token as mod, struct has to have pub in front of every its prop that should be public (in other words, privacy options are all up to coders)
//     pub struct Breakfast {
//         // toast is optional so customers can see the menu of course
//         pub toast: String,
//         // invisible to customers since it's seasonal and therefore there's no way of knowing
//         seasonal_fruit: String,
//     }
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
//     fn fix_incorrect_order() {
//         cook_order();
//         // super refers to its parent mod (in this case, crate root)
//         // super is useful in that it keeps the relationship with the called parent's element even in a different context
//         super::request_serve_order();
//     }
//     fn cook_order() {}
// }
/*******/

// get Rust to read a file of the exact same name and fetch the content
mod back_of_house;
mod front_of_house;

fn request_serve_order() {}

// use makes paths simpler when the namespace chains are long and verbose
// when it comes to functions, it's customary to designate the path to its parent to clarify that the elements are not defined in local and to avoid writing the full path repeatedly
// while as to structs, enums, and other elements, full-path defining is (this has become basic spontaneously)
use crate::back_of_house::Appetizer;
// use crate::front_of_house::hosting;
use std::collections::HashMap;
// they both have Result and thus the name conflicts, so use "as" to create an alias
use std::fmt::Result;
use std::io::Result as IoResult;
// nested use
use std::io::{self, Write};
use std::{cmp::Ordering, iter};

// re-exporting by "pub use"
// with this keyword, external code can also use the path
// restaurant managers ofter think of the team as front of house or back of house, but customers don't do that way (in this case, you and programmers who use your code are likened to managers and customers respectively)
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path: a path starting from the root where main.rs(or lib.rs) is
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path: a path starting from the current path where this is defined
    front_of_house::hosting::add_to_waitlist();
    // with use keyword, it can be shortened
    hosting::add_to_waitlist();

    // Well, I'd like rye toast, please.
    let mut meal: back_of_house::Breakfast = back_of_house::Breakfast::summer("Rye");
    // Ah actually, I'll have wheat toast, please!
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1: back_of_house::Appetizer = Appetizer::Soup;
    let order2: back_of_house::Appetizer = Appetizer::Salad;

    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 2);
}
