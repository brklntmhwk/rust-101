#![allow(unused)]

/* Testing */
// to assert that you get expected outcomes
// Rust does all the type checking and borrow checking, but cannot assert that your code works as expected

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    // a mod for test also follows the visibility rules
    use super::*;

    #[test]
    fn another() {
        // when panicked during the test process, it means failure
        panic!("Wow! I failed to pass the test...")
    }

    /* assert!, assert_eq! and assert_ne! macros */
    #[test]
    fn true_or_false() {
        assert!(true)
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger: Rectangle = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller: Rectangle = Rectangle {
            width: 5,
            height: 1,
        };

        // if the param is true, then the test ends in success
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn add_four() {
        let result: usize = add(2, 2);
        // if left is equal to right, then the test ends in success
        assert_eq!(result, 4);
    }

    #[test]
    fn is_not_equal_to_one() {
        let two: i32 = 2;
        // if left is not equal to right, then the test ends in success
        assert_ne!(two, 1);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // can add a custom failure message
        assert!(
            result.contains("Carol"),
            "Greeting didn't contain name, value was {}",
            result
        );
    }

    // use the should_panic attribute if panic! is what you expect to happen
    // the expected param can contain a full or part of string of the message sent when panicked
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        // if it panics, then the test ends in success
        // this causes a panic related to the expected msg above
        Guess::new(200);
    }

    /* Result<T, E> in tests instead of panic! */
    fn it_works() -> Result<(), String> {
        // to assert that an operation returns an Err variant, use assert!(value.is_err()), not the trailing ? operator
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
