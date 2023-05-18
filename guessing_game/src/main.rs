use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // mut means mutable. Without it, variables are immutable.
    let mut guess = String::new();

    io::stdin()
        // & means reference
        .read_line(&mut guess)
        // read_line method also returns Result type(enum) values, and expect evaluates it
        // if Ok, it returns user input strings, but if Err, it does an error message below
        .expect("Failed to read line");

    // {} is a placeholder that has a variable(variables)
    println!("You guessed: {}", guess);
}
