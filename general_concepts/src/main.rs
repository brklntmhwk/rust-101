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
}
