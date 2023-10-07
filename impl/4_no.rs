extern crate rand;
use rand::Rng;
use std::time::{Instant, Duration};

fn sum_of_squares(input: &[i32]) -> i64 {
    input.iter()
         .map(|&i| i as i64 * i as i64) // Cast to i64 before squaring
         .sum()
}

fn main() {
    let mut rng = rand::thread_rng();
    let numbers: Vec<i32> = (0..1_000_000).map(|_| rng.gen_range(1..101)).collect();

    let start_time = Instant::now();

    let result = sum_of_squares(&numbers);

    let elapsed_time = start_time.elapsed();

    // Convert elapsed time to seconds
    let elapsed_seconds = elapsed_time.as_secs() as f64 + elapsed_time.subsec_nanos() as f64 * 1e-9;

    println!("Sum of squares: {}", result);
    println!("Execution time: {:.6} seconds", elapsed_seconds);
}
