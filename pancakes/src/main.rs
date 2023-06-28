use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// this annotation saves you the process of implementing the HelloMacro trait (the macro does it on behalf)
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
