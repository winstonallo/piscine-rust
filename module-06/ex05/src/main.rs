use rand::prelude::*;
use rayon::prelude::*;
use std::{env::args, iter::*, time::Instant};

fn monte_carlo(n_samples: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let n_inside_circle = (0..n_samples)
        .filter(|_| {
            let x: f64 = rng.gen();
            let y: f64 = rng.gen();
            x * x + y * y <= 1.0
        })
        .count();
    4.0 * (n_inside_circle as f64) / (n_samples as f64)
}

fn monte_carlo_parallel(n_samples: usize) -> f64 {
    let n_inside_circle = (0..n_samples).into_par_iter()
        .filter(|_| {
            let mut rng = rand::thread_rng();
            let x: f64 = rng.gen();
            let y: f64 = rng.gen();
            x * x + y * y <= 1.0
        })
        .count();
    4.0 * (n_inside_circle as f64) / (n_samples as f64)
}

fn main() {
    let n_samples: usize = match args().nth(1) {
        Some(arg) => arg.parse().unwrap_or_else(|_| {
            eprintln!("provide a valid amount of samples.");
            std::process::exit(1);
        }),
        None => {
            eprintln!("cargo run <n_samples>");
            std::process::exit(1);
        }
    };
    let start = Instant::now();
    let pi = monte_carlo(n_samples);
    let duration = start.elapsed();

    println!("single threaded:");
    println!("\tpi: {:.6}", pi);
    println!("\tduration: {:?}", duration);

    let start = Instant::now();
    let pi = monte_carlo_parallel(n_samples);
    let duration = start.elapsed();

    println!("parallel:");
    println!("\tpi: {:.6}", pi);
    println!("\tduration: {:?}", duration);
}
