/* Performance of code with generics */
// no runtime cost!
// using generic types won't make your program run any slower than it would with concrete types thanks to what's called Monomorphization
// Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

// a specific func that only accepts i32 type as a param
#![allow(unused)]

/* In function definitions */
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest: &i32 = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn largest_char(list: &[char]) -> &char {
    let mut largest: &char = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// a more generic func that does the same thing as above but accepts generic types (both i32 and char types work fine!)
// PartialOrd is a trait, and what T accepts as its type should be restricted to only those that implement PartialOrd
// this is similar to <T extends something> in TypeScript
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/* In struct definitions */
// can add as many generic type params as you like but it might as well be within a few in terms of code readability
#[derive(Debug)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

/* In method definitions */
impl<X1, Y1> Point<X1, Y1> {
    fn x(&self) -> &X1 {
        &self.x
    }
    fn y(&self) -> &Y1 {
        &self.y
    }
    // explicitly indicated that X2 and Y2 are different from X1 and Y2 respectively
    // X2 and Y2 are only relevant to the func
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

/* In enum definitions */
// this is exactly what's used to open a file by using std::fs::File
enum Option<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];

    let result: &i32 = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];

    let result: &char = largest(&char_list);
    println!("The largest char is {}", result);

    let integer: Point<i32, i32> = Point { x: 5, y: 10 };
    let float: Point<f64, f64> = Point { x: 1.0, y: 4.0 };

    let p1: Point<i32, f64> = Point { x: 5, y: 10.4 };
    let p2: Point<&str, char> = Point { x: "Hello", y: 'c' };
    let p3: Point<i32, char> = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
