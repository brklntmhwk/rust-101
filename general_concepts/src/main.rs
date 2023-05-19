fn main() {
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
}
