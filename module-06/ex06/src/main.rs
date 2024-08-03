use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
    task_sender: mpsc::Sender<Task>,
    should_stop: Arc<Mutex<bool>>,
}

impl ThreadPool {
    pub fn new(thread_count: usize) -> Self {
        let (task_sender, task_receiver): (mpsc::Sender<Task>, mpsc::Receiver<Task>) = mpsc::channel();
        let task_receiver = Arc::new(Mutex::new(task_receiver));
        let should_stop = Arc::new(Mutex::new(false));

        let mut threads = Vec::with_capacity(thread_count);
        for _ in 0..thread_count {
            let task_receiver = Arc::clone(&task_receiver);
            let should_stop = Arc::clone(&should_stop);
            threads.push(thread::spawn(move || loop {
                let task = {
                    let lock = task_receiver.lock().unwrap();
                    lock.recv()
                };

                match task {
                    Ok(task) => task(),
                    Err(_) => {
                        if *should_stop.lock().unwrap() {
                            break;
                        }
                    }
                }
            }));
        }

        ThreadPool {
            threads,
            task_sender,
            should_stop,
        }
    }

    pub fn spawn_task<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.task_sender.send(Box::new(task)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        {
            let mut should_stop = self.should_stop.lock().unwrap();
            *should_stop = true;
        }

        for _ in &self.threads {
            self.task_sender.send(Box::new(|| {})).unwrap();
        }

        while let Some(thread) = self.threads.pop() {
            if let Err(_) = thread.join() {
                panic!("A thread has panicked!");
            }
        }
    }
}

use std::env;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 24\r\n\r\nThis page does not exist :(";

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send response: {}", e);
        return;
    }

    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush stream: {}", e);
        return;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <address:port>", args[0]);
        return;
    }

    let address = &args[1];
    let listener = TcpListener::bind(address).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.spawn_task(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }
}
