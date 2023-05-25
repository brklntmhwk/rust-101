#![allow(unused)]

/* Struct definition */
// The notation is quite similar to interface of TypeScript
// In Rust, the whole instance must be mutable (Cannot make a part of fields immutable)
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    /* Struct instantiation */
    {
        let mut user1: User = User {
            email: String::from("someone@example.com"),
            username: String::from("some_username123"),
            active: true,
            sign_in_count: 1,
        };
        println!(
            "user1.email: {}, user1.username: {}, user1.active: {}, user1.sign_in_count: {}",
            user1.email, user1.username, user1.active, user1.sign_in_count
        );
        user1.email = String::from("another_email@example.com");
        println!("user1.email(modified): {}", user1.email);
        let user2: User = build_user(
            String::from("someone@example.com"),
            String::from("some_username123"),
        );
        println!("user2.username: {}", user2.username);
        // Like a spread operator of JavaScript, there's a similar one called Struct update syntax
        // It's useful when you want to change some of fields but not all
        let user3: User = User {
            email: String::from("another@example.com"),
            username: String::from("another_username567"),
            ..user2
        };
    }

    /* Tuple structs */
    // Unlike structs, they don't have the names of fields but only the type definitions of them
    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);
        // These are different in types despite their fields' type definitions being exactly the same
        let black: Color = Color(0, 0, 0);
        let origin: Point = Point(0, 0, 0);
        println!("black.1: {}", black.1);
        println!("origin.1: {}", origin.1);
    }
}

// This func explicitly designates parameters as String (no references) because the User instance is supposed to own the data as long as it's valid
fn build_user(email: String, username: String) -> User {
    User {
        // If the name of parameters and that of fields are the same, this shorthand notation works
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
