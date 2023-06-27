use advanced_traits::{Animal, Dog, Human, OutlinePrint, Pilot, Point, Wizard, Wrapper};

fn main() {
    /* Fully qualified syntax for disambiguation */
    // specify which trait's fly method you wan to call
    let person: Human = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // with an associated func that is not a method and doesn't have a self param, the above technique doesn't work throwing a compilation error
    // println!("A baby dog is called a {}", Animal::baby_name());

    // to avoid the error, use a syntax for disambiguation like this: <Type as Trait>::func(...);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let pt: Point = Point { x: 5, y: 15 };
    println!("pt: {:?}", <Point as OutlinePrint>::outline_print(&pt));

    let w: Wrapper = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
