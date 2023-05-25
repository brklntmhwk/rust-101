#![allow(unused)]

use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    let greeting_file_result: Result<File, std::io::Error> = File::open("hello.txt");

    /* match arms */
    // with multiple match arms, you can handle a variety of result cases
    // to every possible case that could fail, add a match arm to handle an error situation
    // a bit verbose
    {
        let greeting_file: File = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem opening the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };
    }
    /* closures */
    // this one is more concise using closures (unwrap_or_else)
    // by the same token, add unwrap_or_else to every possibly failing case
    {
        let greeting_file: File =
            File::open("hello.txt").unwrap_or_else(|error: std::io::Error| {
                if error.kind() == ErrorKind::NotFound {
                    File::create("hello.txt").unwrap_or_else(|error: std::io::Error| {
                        panic!("Problem creating the file: {:?}", error);
                    })
                } else {
                    panic!("Problem opening the file: {:?}", error);
                }
            });
    }
    /* shortcuts */
    // unwrap(): returns the value inside the Ok if the Result value is the Ok variant, while calling the panic! if it's the Err variant
    {
        let greeting_file: File = File::open("hello.txt").unwrap();
    }
    // expect(): also lets you choose the panic! error msg with the unwrap feature
    // this one is more common than unwrap() or panic! macro
    {
        let greeting_file: File =
            File::open("hello.txt").expect("hello.txt should be included in this project");
    }
    /* propagating errors */
    // instead of handling the error inside the func itself, you can return the error to the calling code so that it can decide what to do next
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let username_file_result: Result<File, io::Error> = File::open("hello.txt");

            let mut username_file: File = match username_file_result {
                Ok(file) => file,
                // instead of calling the panic! macro, returns the Err value
                // need to explicitly add return because it's not the end of the func here
                Err(e) => return Err(e),
            };

            let mut username: String = String::new();

            // finally, it returns the username read from the input file (String) if ok, otherwise it does the Err value (io::Error)
            match username_file.read_to_string(&mut username) {
                Ok(_) => Ok(username),
                // needn't to explicitly add return because it's the end of the func here
                Err(e) => Err(e),
            }
        }
    }
    // a shortcut for propagating errors: the ? operator
    // this does the same thing as the above func, but is simpler
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            // the ? operator works the same way as the match expression except that it goes through the from func defined in the From trait in the std lib, which is used to convert values from one type into another
            // this is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons
            let mut username_file: File = File::open("hello.txt")?;
            let mut username: String = String::new();
            username_file.read_to_string(&mut username)?;
            // more concisely, use method chains ↓
            // File::open("hello.txt")?.read_to_string(&mut username)?;
            Ok(username)
        }
    }
    // even more concisely, use fs::read_to_string instead
    // it does all the error handling explained above behind the scene
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
    }
    // in the case where the ? operator is used on Option with an Option<T> returned
    // * the ? operator in a func expects its return type and what it's added to to be the same
    // e.g. cannot mix and match Result and Option<T>
    {
        fn last_char_of_first_line(text: &str) -> Option<char> {
            // lines() returns an iterator over the lines in the string, and then next() is very similar to the same one of JS
            // since next() might be empty (Some or None), it needs the trailing ? operator
            text.lines().next()?.chars().last()
        }
    }
    /* When to cause panic! ? Guidelines for error handling */
    // panic!: when your program isn't executable, due to invalid or incorrect values and so on, and therefore the process should be aborted along the way
    // Result: when operations might fail in a way that your code could recover from
}
