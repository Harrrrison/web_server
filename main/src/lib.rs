use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

struct Job;

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, reciver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        let reciver = Arc::new(Mutex::new(reciver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciver)));
        }
        ThreadPool { workers, sender }
    }
}

impl Worker {
    fn new(id: usize, reciver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            reciver;
        });
        //NOTE if the os cannot create any mroe threads
        //this will panic we'd want to use thead::Builder and its spawn method that retus Result

        Worker { id, thread }
    }
}
