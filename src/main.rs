pub mod solutions;

use crate::solutions::{
  even_fibonacci_numbers, largest_palindrome_product, largest_prime_factor,
  largest_product_in_series, mult_three_five, smallest_multiple,
  special_pythagorean_triplet, sum_square_difference, summation_of_primes,
  ten_thousand_first_prime,
};

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
}
