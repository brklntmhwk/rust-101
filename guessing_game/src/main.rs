use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // mut means mutable. Without it, variables are immutable.
        let mut guess: String = String::new();

        io::stdin()
            // & means reference
            .read_line(&mut guess)
            // read_line method also returns Result type(enum) values, and expect evaluates it
            // if Ok, it returns user input strings, but if Err, it does an error message below
            .expect("Failed to read line");

        // If parse fails, it returns Result type so expect can tail
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // .expect("Please type a number!");

        // {} is a placeholder that has a variable(variables)
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
