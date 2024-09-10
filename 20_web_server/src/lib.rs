use std::{
  sync::{mpsc, Arc, Mutex },
  thread,
};

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Option<mpsc::Sender<Job>>,
}

pub struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `build` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { 
          workers, 
          sender: Some(sender) 
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // as_ref() returns an Option<&T> from an Option<T>, effectively borrowing the Option<T> rather than taking ownership of T.
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
      // dropping sender closes the channel, which indicates no more messages will be sent. 
      drop(self.sender.take());

      for worker in &mut self.workers {
        println!("Shutting down worker {}", worker.id); 

        // we need to move the thread out of the worker instance to be able to call join on it
        if let Some(thread) = worker.thread.take() {
          thread.join().unwrap();
        }
      }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
          // The MutexGuard is dropped at the end of the statement, which means the mutex is unlocked immediately after the recv call completes and the job is retrieved.
            let message = receiver.lock().unwrap().recv();

            match message {
              Ok(job) => {
                println!("Worker {} got a job; executing.", id);

                job();
              },
              Err(_) => {
                println!("Worker {} shutting down.", id);
                break;
              }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}