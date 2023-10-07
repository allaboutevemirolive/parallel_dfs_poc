extern crate rand;
use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

const CHUNK_SIZE: usize = 10_000; // Adjust the chunk size as needed
const NUM_THREADS: usize = 4; // Adjust the number of threads as needed

fn generate_random_numbers(num_elements: usize) -> Vec<i32> {
    (0..num_elements)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            rng.gen_range(1..101)
        })
        .collect()
}

fn sum_of_squares(input: &[i32]) -> i64 {
    input
        .par_chunks(CHUNK_SIZE)
        .map(|chunk| {
            chunk
                .par_iter()
                .map(|&i| i as i64 * i as i64) // Cast to i64 before squaring
                .sum::<i64>()
        })
        .sum()
}

fn main() {
    let start_time = Instant::now();

    let num_elements = 100_000_000;
    let numbers = generate_random_numbers(num_elements);

    let result = sum_of_squares(&numbers);

    let elapsed_time = start_time.elapsed();

    // Convert elapsed time to seconds
    let elapsed_seconds = elapsed_time.as_secs() as f64 + elapsed_time.subsec_nanos() as f64 * 1e-9;

    println!("Sum of squares: {}", result);
    println!("Execution time: {:.6} seconds", elapsed_seconds);
}
