extern crate rand;

use rand::Rng;
use std::time::Instant;

const NUM_ELEMENTS: usize = 100_000_000; // Total number of elements

fn generate_random_numbers(num_elements: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..num_elements)
        .map(|_| rng.gen_range(1..101))
        .collect()
}

fn sum_of_squares(input: &Vec<i32>) -> i64 {
    input.iter().map(|&x| x as i64 * x as i64).sum()
}

fn main() {
    let start_time = Instant::now();

    let numbers = generate_random_numbers(NUM_ELEMENTS);
    let result = sum_of_squares(&numbers);

    let elapsed_time = start_time.elapsed();

    // Convert elapsed time to seconds
    let elapsed_seconds =
        elapsed_time.as_secs() as f64 + elapsed_time.subsec_nanos() as f64 * 1e-9;

    println!("Sum of squares: {}", result);
    println!("Execution time: {:.6} seconds", elapsed_seconds);
}
