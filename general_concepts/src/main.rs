fn main() {
    /*** Variables and code readability ***/
    /* mutable/ immutable variables */
    let mut a: &str = "aaaaa";
    println!("before re-substitution, a is: {}", a);
    a = "AAAAA";
    let b: &str = "bbbbb";
    println!("after re-substitution, a is: {}", a);
    println!("b is: {}", b);

    /* shadowing */
    let x: i32 = 5;
    println!("before shadowing, x is: {}", x);
    let x: i32 = x + 1;
    println!("After shadowing, x is: {}", x);
    // enables reuse of the same variable name defined before
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces is: {}", spaces);

    /* constants */
    const MAX_NUMBER: i32 = 100_000;
    println!("MAX_NUMBER is: {}", MAX_NUMBER);

    /*** Data type ***/
    /* Scholar types */
    /* Integer */
    let x: i32 = -1;
    println!("i32 x is: {}", x);
    let x: u32 = 1;
    println!("u32 x is: {}", x);
    /* Floating-point number */
    let x: f64 = 2.0;
    println!("f64 x is: {}", x);
    /* Boolean */
    let t: bool = true;
    println!("t is: {}", t);
    let f: bool = false;
    println!("f is: {}", f);
    /* Character */
    // use single quotation!!
    let c: char = '😎';
    println!("c is: {}", c);
    /* Tuple */
    let tup: (i32, f64, i32) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup: {}, {}, {}", x, y, z);
    println!(
        "tup (access each elem with period): {}, {}, {}",
        tup.0, tup.1, tup.2
    );
    /* Array */
    // is mainly used when you want data to be in stack rather than heap, or it has its fixed length
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0]: {}", arr[0]);
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("months[0]: {}", months[0]);
    /* String literal (&str) */
    // hardcoded and immutable
    let str: &str = "Hello";
    println!("str: {}", str);
    /* String */
    // mutable
    let mut str: String = String::from("hello");
    str.push_str(", world!");
    println!("str: {}", str);

    /*** Arithmetic operation ***/
    let sum: i32 = 5 + 10;
    println!("sum: {}", sum);
    let diff: f64 = 95.5 - 4.3;
    println!("diff: {}", diff);
    let product: i32 = 4 * 30;
    println!("product: {}", product);
    let quotient: f64 = 56.7 / 32.2;
    println!("quotient: {}", quotient);
    let floored: i32 = 2 / 3;
    println!("floored: {}", floored);
    let remainder: i32 = 43 % 5;
    println!("remainder: {}", remainder);

    /*** Function ***/
    // main fn runs first of all
    another_func();
    func_with_param(5);
    let x: i32 = func_with_ret_value();
    println!("func_with_ret_value x: {}", x);

    /*** Control flow ***/
    /* Conditional branches */
    let num: i32 = 5;
    // Unlike JS, all but boolean values cannot automatically be converted to boolean
    // need to explicitly give boolean values
    if num % 5 == 0 {
        println!("num is divisible by 5");
    } else if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 2, 3, 4, or 5");
    }
    let condition: bool = true;
    // if condition is true, then it returns 5, but if not, it does 6
    let num: i32 = if condition { 5 } else { 6 };
    println!("num: {}", num);

    /*** Loop ***/
    /* loop */
    let mut cnt: i32 = 0;
    // 'counting_up is a label
    'counting_up: loop {
        println!("cnt: {}", cnt);
        let mut remaining: i32 = 10;
        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if cnt == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        cnt += 1;
    }

    /* for */
    let elements: [i32; 5] = [10, 20, 30, 40, 50];
    // This is better than while loop with index var in terms of performance
    for elem in elements {
        println!("elem: {}", elem);
    }
    for number in (1..4).rev() {
        println!("number: {}", number);
    }
    println!("LIFTOFF!!");
    /* while */
    let mut num: i32 = 3;
    while num != 0 {
        println!("num: {}", num);
        num -= 1;
    }
    println!("LIFTOFF!!!");
    let mut index: usize = 0;
    while index < 5 {
        println!("elements[index]: {}", elements[index]);
        index += 1;
    }
}

/*** Function ***/
// Where functions are defined doesn't matter when it comes to Rust compiler
fn another_func() {
    println!("Another function!!");
}
fn func_with_param(x: i32) {
    println!("function with param! x: {}", x);
}
fn func_with_ret_value() -> i32 {
    // This is a formula so don't add a trailing semicolon
    99999 % 55555
}
