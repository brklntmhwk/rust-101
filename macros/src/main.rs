#![allow(unused)]

use macros::jeff_vec;

fn main() {
    /* The diff between macros and functions */
    // - the number of type of params specified in advance... func: necessary, macro: not needed
    // e.g.):
    println!(
        "the curly brace(s) could be as many as you like: {} {} {} {} ...",
        1, 2, 3, 4
    );
    // - the timing when it's interpreted by the compiler
    // a func is called at runtime and a trait needs to be implemented at compile time
    // a macro is expanded before the compiler interprets the code so a trait can be implemented on a given type
    // - the timing it should be defined
    // a func can be defined and called anywhere
    // a macro must be defined or brought into scope before called in a file

    // the downside: their definitions are by and large more complex than those of functions because of their meta-programming aspect

    let v = vec![1, 2, 3];
    let v2 = jeff_vec!(1, 2, 3);
}
