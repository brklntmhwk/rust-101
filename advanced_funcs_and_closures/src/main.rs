#![allow(unused)]

fn main() {
    /* Function pointers */
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
    let list_of_numbers = vec![1, 2, 3];
    {
        // with a closure
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    }
    {
        // with an arg
        let list_of_strings: Vec<String> =
            list_of_numbers.iter().map(ToString::to_string).collect(); // use the fully qualified syntax
    }
    {
        // with an enum
        enum Status {
            Value(u32),
            Stop,
        }

        // each enum variant can become an initializer func as well
        let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    }

    /* Returning closures */
    // this throws an error: return type cannot have an unboxed trait object
    // fn returns_closure() -> dyn Fn(i32) -> i32 {
    //     |x| x + 1
    // }
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
