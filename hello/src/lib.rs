#![allow(unused)]

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // explicitly execute the drop method to close the channel so no more messages will be sent
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // the take method takes out the Some variant and leaves None in its place
            // None in this case means the worker has already had its thread cleaned up and no active thread
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// type alias
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // defining the size param as usize eliminates the possibility of it being negative, but it could still be zero, which is valid but doesn't make sense in this case so test it!

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        // Vec::with_capacity is similar to Vec::new but it preallocates space in the vec
        let mut workers = Vec::with_capacity(size); // you know you need to store size elements in the vec in this case, so it's more efficient

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
    pub fn build(size: usize) -> Result<ThreadPool, &'static str /* PoolCreationError */> {
        if size > 0 {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            Ok(ThreadPool {
                workers,
                sender: Some(sender),
            })
        } else {
            Err("Something went wrong when creating a thread pool.")
        }
    }
}

// like kitchen staff members in a restaurant, wait until the customers finalize their orders, and then fulfill them
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread: thread::JoinHandle<()> = thread::spawn(move || loop {
            /*  A graceful exit pattern */
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }

            /*  An ungraceful exit pattern */
            // let job: Box<dyn FnOnce() + Send> = receiver.lock().unwrap().recv().unwrap();
            // println!("Worker {id} got a job; executing.");
            // job();
        });

        // why not go with the while let loop?
        //  let thread = thread::spawn(move || {
        //     while let Ok(job) = receiver.lock().unwrap().recv() {
        //         println!("Worker {id} got a job; executing.");

        //         job();
        //     }
        // });

        // the reason: the diff in how long the lifetime of the val lasts
        // - the Mutex struct has no public unlock method because the ownership of the lock is based on the lifetime of the MutexGuard<T> within the LockResult<MutexGuard<T>> that the lock method returns
        // - and then at compile time, the borrow checker can enforce the rule that a resource guarded by a Mutex cannot be accessed unless you hold the lock
        // - while let (and if let and match), however, doesn't drop temporary vals until the end of the associated block, and therefore other workers can't receive jobs
        // - with let, any temporary vals used in the expression on the right hand side of the equals sign are immediately dropped shortly after the let statement ends, which is why it works in this case

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
