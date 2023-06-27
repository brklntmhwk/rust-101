/* Unsafe Rust */
// Rust's another alter ego that unleashes hidden potentials prohibited by its conservativeness
// unsafe doesn't mean it's necessarily dangerous or it has always memory safety problems
// it still does borrow checking and thus safety is to some extent guaranteed inside of an unsafe block
// it's useful when working with a low-level system programming like an interaction with OS or even writing your own OS

/* 5 superpowers given by Unsafe Rust */
// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of unions

#![allow(unused)]

fn main() {
    /* Dereferencing a raw pointer */
    // Raw pointers:
    // - are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    // - aren’t guaranteed to point to valid memory
    // - are allowed to be null
    // - don’t implement any automatic cleanup
    {
        let mut num = 5;

        // you can create a raw pointer in a safe block because it does no harm to just define but try to access the val (to do so, of course you need to put it in an unsafe block)
        // these both point to the same memory location: num
        let r1: *const i32 = &num as *const i32;
        let r2: *mut i32 = &mut num as *mut i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    /* Calling an unsafe func or method */
    {
        // inside of the unsafe func is effectively an unsafe block
        unsafe fn dangerous() {}

        unsafe {
            dangerous();
        }

        use std::slice;

        // a func containing unsafe parts doesn't need to be called with a mark unsafe
        fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len: usize = values.len();
            // as_mut_ptr lets you access the raw pointer of a slice
            let ptr: *mut i32 = values.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }
    }

    /* Using extern functions to call external code */
    {
        extern "C" {
            fn abs(input: i32) -> i32;
        }

        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    /* Calling Rust functions from other langs */
    {
        // need to disable mangling so it becomes nameable by other langs
        #[no_mangle]
        pub extern "C" fn call_from_c() {
            println!("Just called a Rust function from C!");
        }
    }

    /* Immutable or mutable static vars */
    // combined with the ownership rules of Rust, this is unlike other langs
    // static vars:
    // - only store refs with the 'static lifetime, so Rust can figure out the lifetime without having to annotate it explicitly
    // - have a fixed address in memory, so they always point to the same data without duplication compared to constants
    static HELLO_WORLD: &str = "Hello, world!";
    static mut COUNTER: u32 = 0;

    {
        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }
        add_to_count(3);

        println!("name is: {}", HELLO_WORLD);

        unsafe {
            // trying to access mutable static vars is unsafe given the ownership rules
            println!("COUNTER: {}", COUNTER);
        }
    }

    /* Implementing an unsafe trait */
    {
        unsafe trait Foo {
            fn bar(&self) -> String;
            fn bar_bar(&self, something: &str);
        }

        unsafe impl Foo for i32 {
            fn bar(&self) -> String {
                String::from("bar!")
            }
            fn bar_bar(&self, something: &str) {
                println!("bar bar!: {}", something);
            }
        }
    }
}
