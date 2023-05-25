#![allow(unused)]

use std::vec;

fn main() {
    /* Vec */
    // retains multiple values but only accepts those of one designated type, in which it's different from Array in JS
    {
        // this way, have to define the type explicitly (since Rust cannot gauge what kind of type is to be set to the data otherwise)
        // more to the point, Rust needs to know the type in advance to know how much memory is required on the heap
        let v: Vec<i32> = Vec::new();
        // this is more common with a useful macro
        let mut v: Vec<i32> = vec![1, 2, 3];
        // like Array in JS
        v.push(5);
        v.push(6);
        v.push(7);
    }
    // values inside also drop when getting out of the scope
    {
        let v: Vec<i32> = vec![1, 2, 3];
    }
    // two ways to refer to the elements
    {
        let v: Vec<i32> = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2]; // throws an error if the index doesn't exist
        let third: Option<&i32> = v.get(2); // returns None if the index doesn't exist
        match v.get(2) {
            Some(third) => println!("third: {}", third),
            None => println!("There's no third element"),
        }
        let v: Vec<i32> = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
        let mut v: Vec<i32> = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
    }
    // what if you want to have some data of different types in one vec?
    // use Vec<enum type> & enum combi!
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        // row is set to be SpreadsheetCell type, so it works fine
        let row: Vec<SpreadsheetCell> = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }

    /* String */
    // String is a wrapper over a Vec<u8>
    // Rustaceans point to both &str and String when they mention on String
    // consists of helpful methods when interpreted as text, and a collection of bytes
    // create a new string
    {
        let mut s: String = String::new();
        let data: &str = "some string contents";
        // these do exactly the same thing!
        let s: String = data.to_string();
        println!("s: {}", s);
        let s: String = String::from(data);
        println!("s: {}", s);
        // all these work because they are properly encoded to UTF-8
        let hellos: Vec<&str> = vec![
            "السلام عليكم",
            "Dobrý den",
            "Hello",
            "שָׁלוֹם",
            "नमस्ते",
            "こんにちは",
            "안녕하세요",
            "你好",
            "Olá",
            "Здравствуйте",
            "Hola",
        ];
        println!("hello in various countries");
        for hello in hellos {
            println!("{}", hello);
        }
    }
    // update a string
    {
        // String can grow in size and its contents can change like that of Vec<T>
        let mut s1: String = String::from("foo!");
        let s2: &str = "bar!";
        // push_str works like push but it doesn't take ownership
        s1.push_str(s2);
        println!("s1: {}", s1);
        // s2 is still alive!
        println!("s2: {}", s2);
        let mut s: String = String::from("lo");
        // push only takes one char as a param
        s.push('l');
        println!("s: {}", s);
        // can use + operator to concatenate strings but it's tricky
        let s1: String = String::from("Hello, ");
        let s2: String = String::from("world!");
        // here, s1's ownership is moved to s3, added by s2 (the latter is a ref so its ownership still exists)
        let s3: String = s1 + &s2;
        // use format macro instead
        let s1: String = String::from("tic");
        let s2: String = String::from("tac");
        let s3: String = String::from("toe");
        let s: String = format!("{}-{}-{}", s1, s2, s3);
        println!("s: {}", s);
        // cannot get access to each string by indices like Vec<T> do because of how Rust store strings in memory
        // in the second example below, the number of bytes it takes to encode "Здравствуйте" in UTF-8 is 24
        // this means indices are not always correlated to a valid Unicode scalar value and therefore often useless returning unexpected values
        // to avoid this, Rust doesn't allow access to strings by their indices
        let hello: String = String::from("Hola");
        println!("hello.len(): {}", hello.len()); // 4
        let hello: String = String::from("Здравствуйте");
        println!("hello.len(): {}", hello.len()); // 24 but not 12
    }

    /* HashMap */
    // a key-value collection like Map in JS
    use std::collections::HashMap;

    // hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must do likewise
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name: String = String::from("Blue");
    // how to get access to the data inside
    let score: i32 = scores.get(&team_name).copied().unwrap_or(0);
    // iterable over each key-value pair in a hash map using a for loop
    for (k, v) in &scores {
        println!("{k}: {v}");
    }
    // entry checks if the key name passed is present in the hash map
    // or_insert inserts the param value into the hashmap if the key doesn't have a value
    scores.entry(String::from("Yellow")).or_insert(50);

    let field_name: String = String::from("Favorite color");
    let field_value: String = String::from("Blue");
    let mut map: HashMap<String, String> = HashMap::new();
    // the hash map will be the owner of these two values from this line
    map.insert(field_name, field_value);

    let text: &str =
        "hello world wonderful world really really beautiful world what an amazing world";
    let mut map: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count: &mut i32 = map.entry(word).or_insert(0);
        *count += 1;
    }
    // the hash map iterated over comes out in an arbitrary order
    println!("map: {:?}", map);
}
