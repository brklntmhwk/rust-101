#![allow(unused)]

use rand::Rng;

/* Enum definition */
// Enumerates all possible variants and works like a namespace

// Directly attach data to each variant to make it simpler (No need to add a struct)
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
// Each variants can take different types
enum InAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}
// (🌟)↓ In this way, IpAddr takes two fields so it's a bit redundant
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}
// (🌟)↑

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// The above does almost the same as this: (but in the below case, each has its own type, which makes it much more complicated and harder to handle)
// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32);

// Like structs, enums can also take impl
impl Message {
    fn call(&self) {
        println!("Hi! Are you calling me?: {:?}", &self);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    let four: IpAddrKind = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;
    println!("four: {:?}", four);
    println!("six: {:?}", six);
    //(🌟) Instantiation goes like this
    let home: IpAddrStruct = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback: IpAddrStruct = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
    //(🌟) Call a func and pass an enum
    route(&IpAddrKind::V4);

    let home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));

    let m: Message = Message::Write(String::from("hello"));
    m.call();

    /* Option<T> (a std lib) */
    /* pub enum Option<T> {
        None,
        Some(T),
    } */
    // The raison d'être: Null safety
    {
        let x: Option<i32> = Some(5);
        let y: i32 = 5;
        let some_char: Option<char> = Some('e');
        // let absent_num: Option<_> = None;

        // This causes an error: cannot add `Option<i8>` to `i8`
        // meaning they are different in types each other therefore type checking and converting Option<T> to T is necessary in advance to perform the operation
        // The very process ensures its null safety before using it
        // let sum = x + y;

        let five: Option<i32> = Some(5);
        let six: Option<i32> = plus_one(five);
        let none: Option<i32> = plus_one(None);
        println!("six: {:?}", six);
        println!("none: {:?}", none);
    }
    /* match's catch-all and the _ placeholder */
    // enhance its exhaustiveness
    {
        let dice_roll: i32 = rand::thread_rng().gen_range(1..7);
        println!("dice_roll: {}", dice_roll);
        match dice_roll {
            1 => {
                println!("You got: {}", 1);
                // This tuple returns nothing
                ()
            }
            2 => {
                println!("You got: {}", 2);
                ()
            }
            // The _ placeholder is similar to the catch-all but doesn't bind to that value
            _ => (),
        }
    }
    /* if let, a concise control flow */
    // When handling only a few variants, match is a bit verbose, so use if let instead
    // less typing, less indentation, and less boilerplate code, but without the exhaustive checking that match enforces
    // decide which one to use depending on a situation, being aware of the trade-off between conciseness and exhaustiveness
    let config_max: Option<u8> = Some(3u8);
    // if config_max is of Option<u8> type,
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // Instead of going like this:
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
}
// This accepts whatever included in IpAddrKind enum
fn route(ip_kind: &IpAddrKind) {
    println!("ip_kind: {:?}", ip_kind);
}
/* match, a control flow construct */
// Works like a coin-sorting machine
fn value_in_cents(coin: Coin) -> u8 {
    // Should cover all possibilities otherwise it throws a compile error (how exhaustive!)
    // Patterns are evaluated in order
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from: {:?} !!", state);
            25
        }
    }
}
// Works with Option<T> as well
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
