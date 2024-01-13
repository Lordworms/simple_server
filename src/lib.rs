use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    worker: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

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
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (st, rs) = mpsc::channel();
        let arc_rs = Arc::new(Mutex::new(rs));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&arc_rs)));
        }
        ThreadPool {
            worker: workers,
            sender: st,
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for w in &mut self.worker {
            println!("shutting down worker {}", w.id);

            if let Some(thread) = w.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, rs: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let msg = rs.lock().unwrap().recv();
            match msg {
                Ok(job) => {
                    println!("worker {id} has a job, start executing!");
                    job();
                }
                Err(_) => {
                    println!("worker {id} disconnected; shutted down");
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
