#[derive(Debug)]
// ↑ an annotation that explicitly indicates that debug be used

struct Rectangle {
    width: u32,
    height: u32,
}

/* Associated functions a.k.a Methods */
// The main reason is for organization, through which future code viewers don't have to bother searching for capabilities of the struct in the vast sea of code
// One struct is allowed to have multiple impl blocks, but that doesn't do anything special (No more than a valid syntax)
impl Rectangle {
    // Enumerate methods of Rectangle here
    // Like a func, ownership and whether mutable or not are selectable
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // It's possible to have functions of the same name as the struct's fields
    fn width(&self) -> bool {
        self.width > 0
    }
    // It's also possible to refer to other objects of the same struct
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // a method without the self param
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3: Rectangle = Rectangle {
        width: 60,
        height: 45,
    };
    // To print out structs, add :? inside curly braces
    println!("rect1: {:?}", rect1);
    // To print out structs that are large enough, add :#? inside curly braces (looks better)
    println!("rect1: {:#?}", rect1);
    // Call area func
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    // Call area method of Rectangle
    // When a method called, Rust automatically adds in &, &mut, or * so the struct obj matches the signature of the method (a.k.a automatic referencing and dereferencing)
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    // No need to explicitly pass the self param since it's self-explanatory
    if rect1.width() {
        println!("The rectangle has a non-zero width: {}", rect1.width);
    }
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // Just like let str1 = String::from("blah-blah-blah");
    let sq: Rectangle = Rectangle::square(3);
    println!("sq: {:?}", sq);
}
// Since width and height are both relevant each other, using tuple struct makes more sense in terms of structure
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
// Instead of receiving params like this:
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
