#![allow(unused)]
fn main() {
    /* Scope */
    {
        let s: String = String::from("hello");
    }
    // Here, Rust automatically calls drop func that returns occupied memories used in the above scope to OS

    /* Move (instead of shallow copy) */
    {
        let s1: String = String::from("hello");
        // Here, data of s1 is copied to s2 meaning that the pointer, length, capacity located in Stack are copied
        // On top of that, to avoid a double free error and ensure memory safety, s1 is invalidated regarded unnecessary anymore (because of this, it's called Move)
        // This happens because Rust judges that String takes up an enough amount of capacity to call large.
        let s2: String = s1;
        // Given the explanation above, this causes an error!
        // println!("s1: {}", s1);

        /* Clone a.k.a deep copy */
        let s1: String = String::from("hello");
        // clone does deep copy meaning that it does copy heap data
        let s2: String = s1.clone();
    }

    /* What if Stack-only data is copied? */
    {
        let x: i32 = 5;
        let y: i32 = x;
        // Move doesn't happen since integer is kept in Stack and thus swift enough to be copied
        // For this reason, Rust judges that it doesn't have to do Move
        // In other words, there's no diff between shallow copy and deep one in this case
        // e.g. u32, bool, f64, char, etc...
        println!("x: {}, y: {}", x, y);
    }

    /* Ownership and functions */
    // Semantically, passing params to a function and substituting values for vars are similar
    {
        let s: String = String::from("Goodbye");
        //  Move happens and s is no longer valid below
        takes_ownership(s);
        // This causes an error
        // println!("s: {}", s);
        let x: i32 = 5;
        // Move happens but i32 has Copy trait so x is still valid
        makes_copy(x);
        // when a function returns something, Move occurs as well and rets is now valid
        let rets: String = gives_ownership();
        // s2 goes into a scope
        let s2: String = String::from("Hey, Hello!");
        // s2 is moved to takes_and_gives_back func and ret value also moves to s3
        let s3: String = takes_and_gives_back(s2);
        // This causes an error
        // println!("s2: {}", s2);
        println!("s3: {}", s3);
    }

    /* References & borrowing */
    // Without move happened and ownership transferred, references make it possible to use objects (variables, functions etc..) but modification isn't allowed by default
    {
        let s1: String = String::from("Hello");
        // & is a symbol of referencing
        let len: usize = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);

        /* Mutable references */
        // Like variables, references also take mutable/immutable options
        let mut s: String = String::from("hello");
        change(&mut s);
        // In a mutable mode, referencing should be only one at a time since it's supposed to be the same constantly from other's view whether mutable or immutable (To prevent data races and keep it consistent, this is very important)
        // This works fine, with more than one mutable refs created but separated by a scope
        {
            let r1 = &mut s;
        }
        let r2 = &mut s;
    }

    /* The Slice type */
    //
    {
        let mut s: String = String::from("Hello world");
        // &String[starting_index..ending_index+1]
        let hello: &str = &s[0..5];
        //Same as: let hello = &s[..5];
        let world: &str = &s[6..11];
        //Same as: let world = &s[6..];
        //Same as: let world = &s[6..s.len()];
        let word: &str = first_word(&s);
        //This causes an error: cannot borrow `s` as mutable because it is also borrowed as immutable
        // s.clear();
        println!("word: {}", word);
        let my_string_literal: &str = "Hello world literal";
        // This works fine on slices of string literals
        let word: &str = first_word(&my_string_literal[..]);
        println!("word (String literals with slices): {}", word);
        // This works fine too because string literals are already string slices
        let word: &str = first_word(my_string_literal);
        println!("word (String literals without slices): {}", word);
        // Array can also be sliced
        let arr = [1, 2, 3, 4, 5];
        let arr_slice = &arr[1..3];
        println!("The length of arr_slice: {}", arr_slice.len());
        println!("arr_slice[0]: {}", arr_slice[0]);
        println!("arr_slice[1]: {}", arr_slice[1]);
    }
}

fn takes_ownership(some_string: String) {
    println!("some_string: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("Reply: Hello!");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
// Instead of doing like this:
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // first_word returns the first word before the first empty space from the beginning if it exists
        if item == b' ' {
            return &s[0..i];
        }
    }
    // Otherwise, it returns the whole string
    &s[..]
}
