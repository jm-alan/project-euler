pub mod solutions;

use std::process::{Command, Output};

use crate::solutions::{
  even_fibonacci_numbers, highly_divisible_triangular_number,
  largest_palindrome_product, largest_prime_factor, largest_product_in_grid,
  largest_product_in_series, lattice_paths, longest_collatz_sequence,
  maximum_path_sum_i, maximum_path_sum_ii, mult_three_five,
  number_letter_counts, smallest_multiple, special_pythagorean_triplet,
  sum_square_difference, summation_of_primes, ten_thousand_first_prime,
};

fn js_run(filename: &str) -> String {
  let mut js_runner = Command::new("node");
  js_runner.current_dir("./js");
  let Output {
    status: _,
    stdout,
    stderr: _,
  } = js_runner
    .arg(filename)
    .output()
    .expect(&format!("Failed to run node ./js/{filename}"));
  std::str::from_utf8(&stdout)
    .expect(&format!("Failed to parse output from node ./js/{filename}"))
    .trim()
    .to_string()
}

fn main() {
  println!("1. Multiples of 3 or 5: {}", mult_three_five());
  println!("2. Even Fibonacci numbers: {}", even_fibonacci_numbers());
  println!("3. Largest prime factor: {}", largest_prime_factor());
  println!(
    "4. Largest palindrome product: {}",
    largest_palindrome_product()
  );
  println!("5. Smallest multiple: {}", smallest_multiple());
  println!("6. Sum square difference: {}", sum_square_difference());
  println!("7. 10001st prime: {}", ten_thousand_first_prime());
  println!(
    "8. Largest product in a series: {}",
    largest_product_in_series()
  );
  println!(
    "9. Special pythagorean triplet: {}",
    special_pythagorean_triplet()
  );
  println!("10. Summation of primes: {}", summation_of_primes());
  println!("11. Largest product in grid: {}", largest_product_in_grid());
  println!(
    "12. Highly divisible triangular number: {}",
    highly_divisible_triangular_number()
  );
  println!("13. Large sum: {}", js_run("thirteen"));
  println!(
    "14. Longest Collatz sequence: {}",
    longest_collatz_sequence()
  );
  println!("15. Lattice paths: {}", lattice_paths());
  println!("16. Power digit sum: {}", js_run("sixteen"));
  println!("17. Number letter counts: {}", number_letter_counts());
  println!("18. Maximum path sum I: {}", maximum_path_sum_i());
  println!("67. Maximum path sum II: {}", maximum_path_sum_ii());
}
