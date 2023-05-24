use crate::module1::module1_1::Module1_1;

// modules are not only useful in arranging code, but also in defining privacy boundaries with the explicit pub
pub mod module1;

fn main() {
    let mod_1: Module1_1 = Module1_1 {};
    println!("mod_1: {:?}", mod_1);
}
