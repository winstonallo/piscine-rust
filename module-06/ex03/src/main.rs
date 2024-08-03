use std::{
    env,
    io::stdin,
    sync::{Arc, Mutex},
    thread::{self, sleep},
};

pub struct Philosopher {
    ideas: Vec<String>,
    brain_size: usize,
}

impl Philosopher {
    pub fn new(brain_size: usize) -> Self {
        Self {
            ideas: vec![],
            brain_size,
        }
    }

    pub fn add_idea(&mut self, idea: String) {
        if self.ideas.len() < self.brain_size {
            self.ideas.push(idea);
        } else {
            println!("the philosopher's head is full")
        }
    }
}

pub fn start(philosopher: Arc<Mutex<Philosopher>>) {
    thread::spawn(move || {
        loop {
            let mut philosopher = philosopher.lock().unwrap();
            if !philosopher.ideas.is_empty() {
                println!(
                    "the philosopher is thinking about {}",
                    philosopher.ideas.pop().unwrap()
                );
                drop(philosopher);
                sleep(std::time::Duration::from_secs(5));
            } else {
                drop(philosopher);
                sleep(std::time::Duration::from_millis(100));
            }
        }        
    });
}

fn main() {
    let brain_size: usize = match env::args().nth(1) {
        Some(arg) => arg.parse().unwrap_or_else(|_| {
            eprintln!("Provide a valid number for brain size.");
            std::process::exit(1);
        }),
        None => {
            eprintln!("<program> <brain_size>");
            std::process::exit(1);
        }
    };

    let philosopher = Arc::new(Mutex::new(Philosopher::new(brain_size)));
    let philosopher_clone = Arc::clone(&philosopher);

    start(Arc::clone(&philosopher_clone));

    let mut input = String::new();
    loop {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        let mut philosopher = philosopher_clone.lock().unwrap();
        philosopher.add_idea(input);
    }
}
