use std::time::Instant;

const NUM_ELEMENTS: usize = 100_000_000; // Total number of elements

fn generate_fixed_numbers(num_elements: usize) -> Vec<i64> {
    (0..num_elements).map(|i| i as i64).collect()
}

fn sum_of_squares(input: &[i64]) -> i128 {
    input.iter().map(|&x| x as i128 * x as i128).sum()
}

fn main() {
    let start_time = Instant::now();

    let numbers = generate_fixed_numbers(NUM_ELEMENTS);
    let result = sum_of_squares(&numbers);

    let elapsed_time = start_time.elapsed();

    // Convert elapsed time to seconds
    let elapsed_seconds =
        elapsed_time.as_secs() as f64 + elapsed_time.subsec_nanos() as f64 * 1e-9;

    println!("Sum of squares (sequential): {}", result);
    println!("Execution time (sequential): {:.6} seconds", elapsed_seconds);
}
