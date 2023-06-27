/* Associated types */
// connect a type placeholder with a trait so that the trait method definitions can use these placeholder types in their signatures
// while generics require annotations of the types in each implementation and let implementations occur multiple times, associated types require the type of Item only once

use std::ops::Add;
// <PlaceholderType = ConcreteType> syntax means a default generic type param (this saves you code and a bit of time most of the time when you don't have to specify the extra param)

// Add trait is like this:
// trait Add<Rhs = Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

struct Millimeters(u32);
struct Meters(u32);

// set the generic type to Meters instead of using the default
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

pub struct Human;

impl Human {
    pub fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

pub trait Animal {
    fn baby_name() -> String;
}

pub struct Dog;

impl Dog {
    pub fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

/* Using supertraits to require one trait's functionality within another trait */
use std::fmt;

pub trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

/* Using the newtype pattern to implement external traits on external types */
// the combi of external traits and external types is not allowed (to implement a trait on a type, either of them must be local to your crate)
// as a workaround to this, use the newtype pattern

// the downside: it doesn't have the methods of the value it's holding (in this case Vec<String>)
// Conversely, it means you can customize which methods to add manually
pub struct Wrapper(pub Vec<String>);

// this causes an error
// output -> only traits defined in the current crate can be implemented for types defined outside of the crate
// define and implement a trait or new type instead
// impl fmt::Display for Vec<String> {}

// go with the newtype instead like this:
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
