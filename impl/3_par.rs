use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

fn sum_of_squares(input: &[i32]) -> i64 {
    input.par_iter()
         .map(|&i| i as i64 * i as i64) // Cast to i64 before squaring
         .sum()
}

fn main() {
    let mut rng = rand::thread_rng();
    let numbers: Vec<i32> = (0..1_000_000).map(|_| rng.gen_range(1..101)).collect();
    
    let start_time = Instant::now();

    let result = sum_of_squares(&numbers);

    // let elapsed_time = start_time.elapsed();

    // println!("Sum of squares: {}", result);
    // println!("Execution time: {:?}", elapsed_time);

    let elapsed_time = start_time.elapsed();

    // Convert elapsed time to seconds
    let elapsed_seconds = elapsed_time.as_secs() as f64 + elapsed_time.subsec_nanos() as f64 * 1e-9;

    println!("Sum of squares: {}", result);
    println!("Execution time: {:.6} seconds", elapsed_seconds);
}