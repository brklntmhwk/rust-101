#![allow(unused)]

/* Usage of three smart pointers */
// Box<T> ...
// Rc<T> ...
// RefCell<T> ...

/* What's different from a raw pointer? */
// operate based on internal logic that a programmer writes

/* Box<T> is a pointer that tells the Rust compiler how much space it needs */
// it provides only the indirection and heap allocation
// a pointer's size does not change based on the amount of data it's pointing to
enum List1 {
    Cons1(i32, Box<List1>),
    Nil1,
}

use crate::List1::{Cons1, Nil1};

/* Rc<T> is an abbreviation for reference counting and keeps track of the number of refs to a value to judge whether the val is still in use */
// this can be likened to a situation where family members watch TV in a living room; When anyone of them enters to watch TV, they turn it on; Others can come into the room and watch it; When the last one of them leaves the room, they have to turn it off.
// allows you to clone other Rc smart pointers that have the ability to immutably borrow the data that was placed on the heap (e.g. clone())
enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
}

use crate::List2::{Cons2, Nil2};

#[derive(Debug)]
enum List3 {
    Cons3(Rc<RefCell<i32>>, Rc<List3>),
    Nil3,
}

use crate::List3::{Cons3, Nil3};

// if you want manually clean up a val before it reaches the end of the main func, use this like drop(sth)
use std::cell::RefCell;
use std::mem::drop;
use std::ops::Deref;
use std::rc::{Rc, Weak};

/* My own smart pointer */
struct MyBox<T>(T);

// This doesn't work as the default Box<T> does because it doesn't have the Deref trait
// type `MyBox<{integer}>` cannot be dereferenced
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)

//     }
// }
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // this returns a ref to the value you want to access with the "*" operator
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // weak ref <---> string ref
    children: RefCell<Vec<Rc<Node>>>,
}

// https://tourofrust.com/103_ja.html
struct Pie {
    slices: u8,
}

impl Pie {
    fn eat_slice(&mut self, name: &str) {
        println!("{} took a slice!", name);
        self.slices -= 1;
    }
}

struct SeaCreature {
    name: String,
    pie: Rc<RefCell<Pie>>,
}

impl SeaCreature {
    fn eat(&self) {
        // use smart pointer to pie for a mutable borrow
        let mut p = self.pie.borrow_mut();
        // take a bite!
        p.eat_slice(&self.name);
    }
}

fn main() {
    {
        // putting the recursive cons in Box, the seemingly infinite (to the Rust compiler) recursion becomes manifest in its size
        let list = Cons1(1, Box::new(Cons1(2, Box::new(Cons1(3, Box::new(Nil1))))));

        /* Deref: dereference */
        // the operator "*" means dereferencing
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        // here, *y lets Rust execute code like this: *(y.deref())
        assert_eq!(5, *y);
    }

    {
        /* Implicit deref coercions */
        // when implementing the Deref trait, Rust takes on those processes that turn &Mybox<String> into &String, and then &str so that the ret value's type matches that of the called func's arg
        let m = Box::new(String::from("Rust"));
        hello(&m);

        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }

    {
        /* Rc<T>, the ref counted smart pointer */
        // here Rc is pointing to a List2 with the ref count increasing (1, 2, 3, ...)
        let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
        println!("count after creating a = {}", Rc::strong_count(&a)); // 1
        let b = Cons2(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a)); // 2
        {
            let c = Cons2(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a)); // 3
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
    }

    {
        /* the combi of Rc<T> and RefCell<T> */
        // enables you to have multiple owners of mut data allowing multiple smart pointers to borrow, whether mutably or immutably, the same data struct
        let value: Rc<RefCell<i32>> = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons3(Rc::clone(&value), Rc::new(Nil3)));

        let b = Cons3(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons3(Rc::new(RefCell::new(4)), Rc::clone(&a));

        println!("a before = {:?}", a);
        println!("b before = {:?}", b);
        println!("c before = {:?}", c);

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }

    {
        let pie = Rc::new(RefCell::new(Pie { slices: 8 }));
        // ferris and sarah are given clones of smart pointer to pie
        let ferris = SeaCreature {
            name: String::from("ferris"),
            pie: pie.clone(),
        };
        let sarah = SeaCreature {
            name: String::from("sarah"),
            pie: pie.clone(),
        };
        // Now, you guys can eat a slice of the pie!!
        ferris.eat(); // the pie lost one of itself
        sarah.eat(); // the pie lost one of itself

        // Let's see how many left..
        let p = pie.borrow();
        println!("{} slices left", p.slices); // must be 6
    }

    {
        /* Weak<T> */
        /* Weak refs don't express an ownership relationship and their count doesn't affect when an Rc<T> instance is cleaned up */
        // to create a tree data structure while avoiding reference cycles
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
