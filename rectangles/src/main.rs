#[derive(Debug)]
// ↑ an annotation that explicitly indicates that debug be used

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // To print out structs, add :? inside curly braces
    println!("rect1: {:?}", rect1);
    // To print out structs that are large enough, add :#? inside curly braces (looks better)
    println!("rect1: {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}
// Since width and height are both relevant each other, using tuple struct makes more sense in terms of structure
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
// Instead of receiving params like this:
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
