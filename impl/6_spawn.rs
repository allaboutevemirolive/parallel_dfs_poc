extern crate rand;
use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

const CHUNK_SIZE: usize = 10_000; // Adjust the chunk size as needed
const NUM_THREADS: usize = 4; // Adjust the number of threads as needed

fn sum_of_squares(input: &[i32]) -> i64 {
    let chunks: Vec<_> = input.chunks(CHUNK_SIZE).collect();

    chunks
        .par_iter()
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

    let mut handles = vec![];
    for _ in 0..NUM_THREADS {
        let handle = std::thread::spawn(|| {
            let mut rng = rand::thread_rng();
            (0..(100_000_000 / NUM_THREADS))
                .map(|_| rng.gen_range(1..101))
                .collect::<Vec<i32>>()
        });
        handles.push(handle);
    }

    let numbers: Vec<i32> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .flatten()
        .collect();

    let result = sum_of_squares(&numbers);

    let elapsed_time = start_time.elapsed();

    // Convert elapsed time to seconds
    let elapsed_seconds = elapsed_time.as_secs() as f64 + elapsed_time.subsec_nanos() as f64 * 1e-9;

    println!("Sum of squares: {}", result);
    println!("Execution time: {:.6} seconds", elapsed_seconds);
}
