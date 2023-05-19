#![allow(unused)]
fn main() {
    /* Scope */
    {
        let s = String::from("hello");
    }
    // Here, Rust automatically calls drop func that returns occupied memories used in the above scope to OS

    /* Move (instead of shallow copy) */
    let s1 = String::from("hello");
    // Here, data of s1 is copied to s2 meaning that the pointer, length, capacity located in Stack are copied
    // On top of that, to avoid a double free error and ensure memory safety, s1 is invalidated regarded unnecessary anymore (because of this, it's called Move)
    // This happens because Rust judges that String takes up an enough amount of capacity to call large.
    let s2 = s1;
    // Given the explanation above, this causes an error!
    // println!("s1: {}", s1);

    /* Clone a.k.a deep copy */
    let s1 = String::from("hello");
    // clone does deep copy meaning that it does copy heap data
    let s2 = s1.clone();

    /* What if Stack-only data is copied? */
    let x = 5;
    let y = x;
    // Move doesn't happen since integer is kept in Stack and thus swift enough to be copied
    // For this reason, Rust judges that it doesn't have to do Move
    // In other words, there's no diff between shallow copy and deep one in this case
    // e.g. u32, bool, f64, char, etc...
    println!("x: {}, y: {}", x, y);

    /* Ownership and functions */
    // Semantically, passing params to a function and substituting values for vars are similar
    let s = String::from("Goodbye");
    //  Move happens and s is no longer valid below
    takes_ownership(s);
    // This causes an error
    // println!("s: {}", s);
    let x = 5;
    // Move happens but i32 has Copy trait so x is still valid
    makes_copy(x);
    // when a function returns something, Move occurs as well and rets is now valid
    let rets = gives_ownership();
    // s2 goes into a scope
    let s2 = String::from("Hey, Hello!");
    // s2 is moved to takes_and_gives_back func and ret value also moves to s3
    let s3 = takes_and_gives_back(s2);
    // This causes an error
    // println!("s2: {}", s2);
    println!("s3: {}", s3);
}

fn takes_ownership(some_string: String) {
    println!("some_string: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Reply: Hello!");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
