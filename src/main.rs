extern crate rand;
use rand::Rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

const CHUNK_SIZE: usize = 10_000; // Adjust the chunk size as needed
const NUM_THREADS: usize = 10; // Adjust the number of threads as needed
const NUM_ELEMENTS: usize = 100_000_000; // Total number of elements

fn generate_random_numbers(num_elements: usize) -> Vec<i32> {
    (0..num_elements)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            rng.gen_range(1..101)
        })
        .collect()
}

fn sum_of_squares_parallel(input: Arc<Vec<i32>>, num_threads: usize) -> i64 {
    let chunk_size = CHUNK_SIZE;
    let results = Arc::new(Mutex::new(vec![0; num_threads]));

    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let input = input.clone();
            let results = results.clone();

            thread::spawn(move || {
                let start = i * chunk_size;
                let end = start + chunk_size.min(input.len() - start);

                let sum = input[start..end]
                    .iter()
                    .map(|&x| x as i64 * x as i64)
                    .sum::<i64>();

                let mut results = results.lock().unwrap();
                results[i] = sum;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    results.iter().sum()
}

fn main() {
    let start_time = Instant::now();

    let numbers = generate_random_numbers(NUM_ELEMENTS);
    let numbers = Arc::new(numbers);

    let result = sum_of_squares_parallel(numbers.clone(), NUM_THREADS);

    let elapsed_time = start_time.elapsed();

    // Convert elapsed time to seconds
    let elapsed_seconds = elapsed_time.as_secs() as f64 + elapsed_time.subsec_nanos() as f64 * 1e-9;

    println!("Sum of squares: {}", result);
    println!("Execution time: {:.6} seconds", elapsed_seconds);
}
