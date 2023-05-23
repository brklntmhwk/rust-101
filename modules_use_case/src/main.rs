use crate::module1::module1_1::Module1_1;

pub mod module1;

fn main() {
    let mod_1: Module1_1 = Module1_1 {};
    println!("mod_1: {:?}", mod_1);
}
