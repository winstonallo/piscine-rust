use std::sync::{Condvar, Mutex};

pub struct RendezVous<T> {
    pair: Mutex<Option<T>>,
    condvar: Condvar,
}

impl<T: Clone> RendezVous<T> {
    pub const fn new() -> Self {
        RendezVous {
            pair: Mutex::new(None),
            condvar: Condvar::new(),
        }
    }

    pub fn wait(&self, value: T) -> T {
        let mut guard = self.pair.lock().unwrap();
        if let Some(val) = guard.take() {
            self.condvar.notify_one();
            val
        } else {
            *guard = Some(value);
            guard.take().expect("Expected a value but found None")
        }
    }

    pub fn try_wait(&self, value: T) -> Result<T, T> {
        let mut guard = self.pair.lock().unwrap();
        if let Some(val) = guard.take() {
            self.condvar.notify_one();
            Ok(val)
        } else {
            *guard = Some(value.clone());
            Err(value)
        }
    }
}

use std::sync::Arc;
use std::thread;

#[allow(dead_code)]
fn main() {
    let rdv = Arc::new(RendezVous::new());

    let rdv1 = Arc::clone(&rdv);
    let handle1 = thread::spawn(move || {
        let res = rdv1.wait(42u32);
        println!("Thread 1 received: {}", res);
    });

    let rdv2 = Arc::clone(&rdv);
    let handle2 = thread::spawn(move || {
        let res = rdv2.wait(21u32);
        println!("Thread 2 received: {}", res);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
