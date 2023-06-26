/* Refutability */
// if a pattern is refutable, it could fail to match for some possible value (e.g. if let, match arms, etc..)
// if a pattern is irrefutable, it will match for any possible value (e.g. let x = 5;)
// where it's supposed to be refutable, a pattern must not be irrefutable, and vice versa

#![allow(unused)]

fn main() {
    {
        /* Matching literals */
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    {
        /* Matching named variables */
        let x = Some(5);
        let y = 10;

        // this is irrefutable and the y in the second condition is different from the above
        // these combined, the pattern matches for the second one; it accepts whatever with the y meaning any value inside a Some
        match x {
            // here the pattern matches for the second one
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {y}", x);
    }
    {
        /* Multiple patterns */
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    {
        /* Matching ranges of values (..=) */
        // since Rust can only tell if a range is empty or not for char or numeric values, this way is only allowed with them
        let x = 5;

        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }

        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }
    {
        /* Destructuring structs */
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p;
        // let Point { x, y } = p; <---- shorthanded
        assert_eq!(0, a);
        assert_eq!(7, b);

        let p = Point { x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }

        /* Destructuring structs and tuples */
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    }
    {
        /* Destructuring enums */
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg_1 = Message::ChangeColor(0, 160, 255);
        let msg_2 = Message::Quit;
        let msg_3 = Message::Move { x: 50, y: 50 };
        let msg_4 = Message::Write(String::from("aaaaa"));

        match msg_4 {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}",)
            }
        }
    }
    {
        /* Destructuring nested structs and enums */
        // even where multiple enums involved, you can do pattern matching with a single match by using this technique
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => (),
        }
    }
    {
        /* Ignoring values*/
        {
            /* An entire value ignored with _ */
            // this is useful when you're implementing a trait, one of whose function's parameter is unused but you need a certain type signature and the function in and of itself
            fn foo(_: i32, y: i32) {
                println!("This code only uses the y parameter: {}", y);
            }

            foo(3, 4);
        }
        {
            /* Parts of a value ignored with a nested _ */
            let mut setting_value = Some(5);
            let new_setting_value = Some(10);

            match (setting_value, new_setting_value) {
                // cannot overwrite it if any val exists
                (Some(_), Some(_)) => {
                    println!("Can't overwrite an existing customized value");
                }
                _ => {
                    setting_value = new_setting_value;
                }
            }

            println!("setting is {:?}", setting_value);
        }
        {
            /* An unused variable ignored with its name started by _ */
            // the diff between _ and _x is that the syntax _x still binds the val to the var, whereas _ doesn't do so at all
            let _x = 5;
            // let y = 10;
        }
        {
            /* Remaining parts of a val ignored with .. */
            struct Point {
                x: i32,
                y: i32,
                z: i32,
            }

            let origin = Point { x: 0, y: 0, z: 0 };

            match origin {
                Point { x, .. } => println!("x is {}", x), // instead of writing like y: _, z: _
            }

            let numbers = (1, 2, 3, 4, 5);

            match numbers {
                (first, .., last) => {
                    println!("Some numbers: {first}, {last}");
                }
            }
        }
    }
    {
        /* Extra conditionals with match guards: additional if */
        // this is useful when you add more complexity to pattern matching
        // the downside is the compiler doesn't check for exhaustiveness when match guard expressions are involved
        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("The number {} is even", x),
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }

        let x = Some(5);
        let y = 10;

        // a match guard enables you to test against the val from the outside
        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {n}"), // using the outer y for evaluation
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {y}", x);

        let x = 4;
        let y = false;

        // with the OR operator |
        match x {
            // if x is either 4, 5 or 6 and y is true, print "yes"
            4 | 5 | 6 if y => println!("yes"), // works like this: (4 | 5 | 6) if y => ...
            _ => println!("no"),
        }
    }
    {
        /* @ Bindings */
        // this lets us create a var that holds a val at the same time as we're testing that value for a pattern match
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {}", id_variable),
            // this doesn't work
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            // this works but matches any value
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}
