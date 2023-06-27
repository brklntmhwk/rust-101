#![allow(unused)]

fn main() {
    /* Type aliases */
    // you can give those existing types another name
    type Kilometers = i32;

    let x = 5;
    let y: Kilometers = 8;

    // this can work but you lose the type checking benefits
    println!("x + y =: {}", x + y);

    // to reduce repetitions and avoid lengthy expressions, it's useful
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    /* The never type */
    // e.g. continue in a match arm returns it
    // why?: the match's return type must be only one and the never type works filling in the Err case to make it consistent with its feature of never having a value
    //   let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    fn bar() -> ! {
        panic!("forever!")
    }

    /* Dynamically sized types and the Sized trait */
    {
        fn generic<T>(t: T) {
            // --snip--
        }
        // the above is actually treated as:
        // fn generic<T: Sized>(t: T) {
        //     // --snip--
        // }
    }
    {
        // generic functions don't work on types that don't have a known size at compile time by default
        // but you can use the following syntax to relax this restriction:
        fn generic<T: ?Sized>(t: &T) {
            // ?Sized means T may or may not be "Sized" (* this syntax is only available for Sized)
            // --snip--
        }
    }
}
