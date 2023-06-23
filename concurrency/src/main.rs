/*   */

// mpsc stands for multiple producer, single consumer
// use std::rc::Rc;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// pub fn spawn<F, T>(f: F) -> JoinHandle<T>
// where
//     F: FnOnce() -> T + Send + 'static,
//     T: Send + 'static,

fn main() {
    {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // when this for loop finishes, the main reaches its end and the process ends even though the thread above is still ongoing
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    {
        let handle: thread::JoinHandle<()> = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // it will wait for its thread to finish
        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }

    {
        /* Message passing to transfer data between threads */
        // a channel has two halves: a transmitter(also called a sender) and a receiver
        // a transmitter: the upstream location where you put rubber ducks into the river
        // a receiver: the place where rubber ducks end up getting to downstream
        let (tx, rx) = mpsc::channel();

        // let a rubber duck go downstream
        thread::spawn(move || {
            let val = String::from("hi");
            // the send func takes ownership of its param
            tx.send(val).unwrap();

            // so this can't work since the val has already been moved
            // println!("val is {}", val);
        });

        // retrieve the duck from the water at the end of the river!
        // now this receiver has taken ownership of it
        let received = rx.recv().unwrap();
        println!("Got: {}", received);

        {
            /* Sending multiple values and seeing the receiver waiting */
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
            });

            for received in rx {
                println!("Got: {}", received);
            }
        }

        {
            /* Creating multiple producers by cloning the transmitter */
            let (tx, rx) = mpsc::channel();

            // to create more transmitters, use the clone method
            let tx1 = tx.clone();
            thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];

                for val in vals {
                    tx1.send(val).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
            });

            thread::spawn(move || {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
            });

            for received in rx {
                println!("Got: {}", received);
            }
        }

        {
            /* Shared-state concurrency */
            /* Mutex, short for Mutual exclusion */
            // a mutex plays a roll as a guardian in allowing access to data from one thread at any time
            // you must follow the two rules:
            // 1. acquire the lock before using the data
            // 2. unlock the data when you're done with the data so that other threads can acquire the lock (this is automatically conducted thanks to the Drop trait the MutexGuard owns)
            // this is likened to a panel discussion where panelists share one mic and have to signal when they want to use it

            // summon a guardian that protects i32 num!!
            let m = Mutex::new(5);

            {
                // ...Can I get a permission to get access to the data? - Sure. here's the key. Please return it when you finish.
                let mut num: std::sync::MutexGuard<'_, i32> = m.lock().unwrap(); // MutexGuard is a smart pointer
                *num = 6;
            }

            println!("m = {:?}", m);

            {
                // Rc<T> can't work in this case
                // get this error: `Rc<Mutex<i32>>` cannot be sent between threads safely (...) the trait `Send` is not implemented for `Rc<Mutex<i32>>`
                // let counter = Rc::new(Mutex::new(0));

                // use Arc<T> instead, which is short for an atomic reference counted
                let counter = Arc::new(Mutex::new(0));
                let mut handles = vec![];

                for _ in 0..10 {
                    let counter = Arc::clone(&counter);
                    // let counter = Rc::clone(&counter);
                    let handle = thread::spawn(move || {
                        let mut num = counter.lock().unwrap();

                        *num += 1;
                    });
                    handles.push(handle);
                }

                for handle in handles {
                    handle.join().unwrap();
                }

                println!("Result: {}", *counter.lock().unwrap());
            }
        }
    }
}
