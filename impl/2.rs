use rand::Rng;
use rayon::prelude::*;

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter()
         .map(|&i| i * i)
         .sum()
}

fn main() {
    let mut rng = rand::thread_rng();
    let numbers: Vec<i32> = (0..1_000).map(|_| rng.gen_range(1..101)).collect();
    
    println!("Generated numbers: {:?}", numbers);

    let result = sum_of_squares(&numbers);

    println!("Sum of squares: {}", result);
}
