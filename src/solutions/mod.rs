mod big_triangle;

use std::{
  collections::HashMap,
  fs::File,
  io::{BufRead, BufReader},
};

#[inline(always)]
fn sum_multiples_below(multiple: u64, mut limit: u64) -> u64 {
  limit -= 1; // guarantees "below" in case limit % multiple == 0
  limit -= limit % multiple;
  limit /= multiple;
  limit *= limit + 1;
  limit /= 2;
  limit *= multiple;
  limit
}

pub fn mult_three_five() -> u64 {
  sum_multiples_below(3, 1000) + sum_multiples_below(5, 1000)
    - sum_multiples_below(15, 1000)
}

pub fn even_fibonacci_numbers() -> u64 {
  let mut storage: [u64; 3] = [1, 1, 2];
  let mut sum: u64 = 0;

  while storage[2] < 4000000 {
    if storage[2] % 2 == 0 {
      sum += storage[2];
    }
    storage[0] = storage[1];
    storage[1] = storage[2];
    storage[2] = storage[0] + storage[1];
  }

  sum
}

#[inline(always)]
fn is_prime(num: u64) -> bool {
  if num < 2 {
    return false;
  } else if num == 2 || num == 3 {
    return true;
  } else if num % 2 == 0 {
    return false;
  } else if num % 3 == 0 {
    return false;
  };

  for i in 3..=((num as f64).sqrt() as u64) {
    if num % i == 0 {
      return false;
    }
  }
  true
}

pub fn largest_prime_factor() -> u64 {
  let mut largest = 1;
  for i in 3..=((600851475143u64 as f64).sqrt() as u64) {
    if 600851475143 % i == 0 && is_prime(i) {
      largest = i
    }
  }
  largest
}

#[inline(always)]
pub fn is_palindrome(mut num: u32) -> bool {
  let mut digits: Vec<u8> = vec![];
  while num > 0 {
    digits.push((num % 10) as u8);
    num /= 10;
  }
  let safe_digit_ceil = digits.len() - 1;
  for i in 0..=safe_digit_ceil {
    if digits[i] != digits[safe_digit_ceil - i] {
      return false;
    }
  }
  true
}

pub fn largest_palindrome_product() -> u32 {
  let mut largest = 10001;
  for i in 100..=999 {
    for j in 100..=999 {
      let product = i * j;
      if product > largest && is_palindrome(product) {
        largest = product;
      }
    }
  }
  largest
}

#[inline(always)]
fn reduce_factors(factors: &mut Vec<u64>) {
  factors.sort_unstable();
  let mut i = 0;
  let mut j = 1;
  loop {
    loop {
      if factors[j] % factors[i] == 0 {
        factors[j] /= factors[i];
      }
      if j == factors.len() - 1 {
        break;
      }
      j += 1;
    }
    i += 1;
    if i == factors.len() - 1 {
      break;
    }
    j = i + 1;
  }
}

pub fn smallest_multiple() -> u64 {
  let mut factors = vec![
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
  ];

  reduce_factors(&mut factors);

  factors.into_iter().reduce(|a, b| a * b).unwrap()
}

pub fn sum_square_difference() -> u64 {
  let sum_squares = {
    let mut sum = 0;
    for i in 1..=100u64 {
      sum += i.pow(2);
    }
    sum
  };
  let square_sum = ((100 * 101) / 2u64).pow(2);
  println!("Sum suqares: {sum_squares}, square sum: {square_sum}");
  square_sum - sum_squares
}

pub fn ten_thousand_first_prime() -> u64 {
  let mut primes_found = 0;
  let mut last_prime = 0;
  let mut current_check = 2;
  loop {
    if primes_found == 10001 {
      break;
    }
    if is_prime(current_check) {
      last_prime = current_check;
      primes_found += 1;
    }
    current_check += 1;
  }
  last_prime
}

const DIGITAL: [u64; 1000] = [
  7, 3, 1, 6, 7, 1, 7, 6, 5, 3, 1, 3, 3, 0, 6, 2, 4, 9, 1, 9, 2, 2, 5, 1, 1, 9,
  6, 7, 4, 4, 2, 6, 5, 7, 4, 7, 4, 2, 3, 5, 5, 3, 4, 9, 1, 9, 4, 9, 3, 4, 9, 6,
  9, 8, 3, 5, 2, 0, 3, 1, 2, 7, 7, 4, 5, 0, 6, 3, 2, 6, 2, 3, 9, 5, 7, 8, 3, 1,
  8, 0, 1, 6, 9, 8, 4, 8, 0, 1, 8, 6, 9, 4, 7, 8, 8, 5, 1, 8, 4, 3, 8, 5, 8, 6,
  1, 5, 6, 0, 7, 8, 9, 1, 1, 2, 9, 4, 9, 4, 9, 5, 4, 5, 9, 5, 0, 1, 7, 3, 7, 9,
  5, 8, 3, 3, 1, 9, 5, 2, 8, 5, 3, 2, 0, 8, 8, 0, 5, 5, 1, 1, 1, 2, 5, 4, 0, 6,
  9, 8, 7, 4, 7, 1, 5, 8, 5, 2, 3, 8, 6, 3, 0, 5, 0, 7, 1, 5, 6, 9, 3, 2, 9, 0,
  9, 6, 3, 2, 9, 5, 2, 2, 7, 4, 4, 3, 0, 4, 3, 5, 5, 7, 6, 6, 8, 9, 6, 6, 4, 8,
  9, 5, 0, 4, 4, 5, 2, 4, 4, 5, 2, 3, 1, 6, 1, 7, 3, 1, 8, 5, 6, 4, 0, 3, 0, 9,
  8, 7, 1, 1, 1, 2, 1, 7, 2, 2, 3, 8, 3, 1, 1, 3, 6, 2, 2, 2, 9, 8, 9, 3, 4, 2,
  3, 3, 8, 0, 3, 0, 8, 1, 3, 5, 3, 3, 6, 2, 7, 6, 6, 1, 4, 2, 8, 2, 8, 0, 6, 4,
  4, 4, 4, 8, 6, 6, 4, 5, 2, 3, 8, 7, 4, 9, 3, 0, 3, 5, 8, 9, 0, 7, 2, 9, 6, 2,
  9, 0, 4, 9, 1, 5, 6, 0, 4, 4, 0, 7, 7, 2, 3, 9, 0, 7, 1, 3, 8, 1, 0, 5, 1, 5,
  8, 5, 9, 3, 0, 7, 9, 6, 0, 8, 6, 6, 7, 0, 1, 7, 2, 4, 2, 7, 1, 2, 1, 8, 8, 3,
  9, 9, 8, 7, 9, 7, 9, 0, 8, 7, 9, 2, 2, 7, 4, 9, 2, 1, 9, 0, 1, 6, 9, 9, 7, 2,
  0, 8, 8, 8, 0, 9, 3, 7, 7, 6, 6, 5, 7, 2, 7, 3, 3, 3, 0, 0, 1, 0, 5, 3, 3, 6,
  7, 8, 8, 1, 2, 2, 0, 2, 3, 5, 4, 2, 1, 8, 0, 9, 7, 5, 1, 2, 5, 4, 5, 4, 0, 5,
  9, 4, 7, 5, 2, 2, 4, 3, 5, 2, 5, 8, 4, 9, 0, 7, 7, 1, 1, 6, 7, 0, 5, 5, 6, 0,
  1, 3, 6, 0, 4, 8, 3, 9, 5, 8, 6, 4, 4, 6, 7, 0, 6, 3, 2, 4, 4, 1, 5, 7, 2, 2,
  1, 5, 5, 3, 9, 7, 5, 3, 6, 9, 7, 8, 1, 7, 9, 7, 7, 8, 4, 6, 1, 7, 4, 0, 6, 4,
  9, 5, 5, 1, 4, 9, 2, 9, 0, 8, 6, 2, 5, 6, 9, 3, 2, 1, 9, 7, 8, 4, 6, 8, 6, 2,
  2, 4, 8, 2, 8, 3, 9, 7, 2, 2, 4, 1, 3, 7, 5, 6, 5, 7, 0, 5, 6, 0, 5, 7, 4, 9,
  0, 2, 6, 1, 4, 0, 7, 9, 7, 2, 9, 6, 8, 6, 5, 2, 4, 1, 4, 5, 3, 5, 1, 0, 0, 4,
  7, 4, 8, 2, 1, 6, 6, 3, 7, 0, 4, 8, 4, 4, 0, 3, 1, 9, 9, 8, 9, 0, 0, 0, 8, 8,
  9, 5, 2, 4, 3, 4, 5, 0, 6, 5, 8, 5, 4, 1, 2, 2, 7, 5, 8, 8, 6, 6, 6, 8, 8, 1,
  1, 6, 4, 2, 7, 1, 7, 1, 4, 7, 9, 9, 2, 4, 4, 4, 2, 9, 2, 8, 2, 3, 0, 8, 6, 3,
  4, 6, 5, 6, 7, 4, 8, 1, 3, 9, 1, 9, 1, 2, 3, 1, 6, 2, 8, 2, 4, 5, 8, 6, 1, 7,
  8, 6, 6, 4, 5, 8, 3, 5, 9, 1, 2, 4, 5, 6, 6, 5, 2, 9, 4, 7, 6, 5, 4, 5, 6, 8,
  2, 8, 4, 8, 9, 1, 2, 8, 8, 3, 1, 4, 2, 6, 0, 7, 6, 9, 0, 0, 4, 2, 2, 4, 2, 1,
  9, 0, 2, 2, 6, 7, 1, 0, 5, 5, 6, 2, 6, 3, 2, 1, 1, 1, 1, 1, 0, 9, 3, 7, 0, 5,
  4, 4, 2, 1, 7, 5, 0, 6, 9, 4, 1, 6, 5, 8, 9, 6, 0, 4, 0, 8, 0, 7, 1, 9, 8, 4,
  0, 3, 8, 5, 0, 9, 6, 2, 4, 5, 5, 4, 4, 4, 3, 6, 2, 9, 8, 1, 2, 3, 0, 9, 8, 7,
  8, 7, 9, 9, 2, 7, 2, 4, 4, 2, 8, 4, 9, 0, 9, 1, 8, 8, 8, 4, 5, 8, 0, 1, 5, 6,
  1, 6, 6, 0, 9, 7, 9, 1, 9, 1, 3, 3, 8, 7, 5, 4, 9, 9, 2, 0, 0, 5, 2, 4, 0, 6,
  3, 6, 8, 9, 9, 1, 2, 5, 6, 0, 7, 1, 7, 6, 0, 6, 0, 5, 8, 8, 6, 1, 1, 6, 4, 6,
  7, 1, 0, 9, 4, 0, 5, 0, 7, 7, 5, 4, 1, 0, 0, 2, 2, 5, 6, 9, 8, 3, 1, 5, 5, 2,
  0, 0, 0, 5, 5, 9, 3, 5, 7, 2, 9, 7, 2, 5, 7, 1, 6, 3, 6, 2, 6, 9, 5, 6, 1, 8,
  8, 2, 6, 7, 0, 4, 2, 8, 2, 5, 2, 4, 8, 3, 6, 0, 0, 8, 2, 3, 2, 5, 7, 5, 3, 0,
  4, 2, 0, 7, 5, 2, 9, 6, 3, 4, 5, 0,
];

pub fn largest_product_in_series() -> u64 {
  let mut largest = DIGITAL[0..13].iter().product();
  for i in 1..986 {
    let sum = DIGITAL[i..(i + 13)].iter().product();
    if sum > largest {
      largest = sum
    }
  }
  largest
}

pub fn special_pythagorean_triplet() -> f64 {
  let mut a: f64 = 1.0;
  let mut b: f64 = 2.0;
  while a < 400.0 {
    while b < 400.0 {
      let a2 = a.powf(2.0);
      let b2 = b.powf(2.0);
      let interstitial_pow = a2 + b2;
      let c = (interstitial_pow as f64).sqrt();
      let sum = a + b + c;
      if sum == 1000.0 {
        return a * b * c;
      }
      b += 1.0;
    }
    a += 1.0;
    b = a + 1.0;
  }
  panic!("Failed to find triple");
}

pub fn summation_of_primes() -> u64 {
  let mut sum = 0;
  for i in 0..2000000 {
    if is_prime(i) {
      sum += i
    }
  }
  sum
}

const GRID: [[u64; 20]; 20] = [
  [
    08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91,
    08,
  ],
  [
    49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62,
    00,
  ],
  [
    81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36,
    65,
  ],
  [
    52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36,
    91,
  ],
  [
    22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13,
    80,
  ],
  [
    24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12,
    50,
  ],
  [
    32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64,
    70,
  ],
  [
    67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94,
    21,
  ],
  [
    24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63,
    72,
  ],
  [
    21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33,
    95,
  ],
  [
    78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56,
    92,
  ],
  [
    16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85,
    57,
  ],
  [
    86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17,
    58,
  ],
  [
    19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55,
    40,
  ],
  [
    04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98,
    66,
  ],
  [
    88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53,
    69,
  ],
  [
    04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76,
    36,
  ],
  [
    20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36,
    16,
  ],
  [
    20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05,
    54,
  ],
  [
    01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67,
    48,
  ],
];

pub fn largest_product_in_grid() -> u64 {
  let mut largest = 0;

  for i in 0..16 {
    for j in 0..20 {
      let v_line = [GRID[i][j], GRID[i + 1][j], GRID[i + 2][j], GRID[i + 3][j]];
      let h_line = [GRID[j][i], GRID[j][i + 1], GRID[j][i + 2], GRID[j][i + 3]];
      let v_product: u64 = v_line.iter().product();
      let h_product: u64 = h_line.iter().product();
      let larger = u64::max(h_product, v_product);
      largest = u64::max(largest, larger);
    }
  }

  for i in 0..16 {
    for j in 0..16 {
      let l_diag = [
        GRID[i][j],
        GRID[i + 1][j + 1],
        GRID[i + 2][j + 2],
        GRID[i + 3][j + 3],
      ];
      let r_diag = [
        GRID[i][j + 3],
        GRID[i + 1][j + 2],
        GRID[i + 2][j + 1],
        GRID[i + 3][j],
      ];
      let l_product: u64 = l_diag.iter().product();
      let r_product: u64 = r_diag.iter().product();
      let larger = u64::max(l_product, r_product);
      largest = u64::max(largest, larger);
    }
  }

  largest
}

#[inline(always)]
fn count_divisors(num: u64) -> u64 {
  let mut divisors = 0;
  for i in 1..=((num as f64).sqrt() as u64) {
    if num % i == 0 {
      divisors += 2
    }
  }
  divisors
}

pub fn highly_divisible_triangular_number() -> u64 {
  let mut next_add = 2;
  let mut current_triangle = 1;
  while count_divisors(current_triangle) <= 500 {
    current_triangle += next_add;
    next_add += 1;
  }
  current_triangle
}

#[inline(always)]
fn get_collatz_length(mut num: u64) -> u64 {
  let mut terms = 1;
  while num != 1 {
    num = if num % 2 == 0 { num / 2 } else { (3 * num) + 1 };
    terms += 1;
  }
  terms
}

pub fn longest_collatz_sequence() -> u64 {
  let mut longest = 0;
  let mut generates_longest = 1;
  for i in 1..1000000 {
    let c_length = get_collatz_length(i);
    if c_length > longest {
      longest = c_length;
      generates_longest = i;
    }
  }
  generates_longest
}

#[inline(always)]
fn calculate_paths_through_point(
  depth: u64,
  position: u64,
  computed: &mut HashMap<(u64, u64), u64>,
) -> u64 {
  if computed.contains_key(&(depth, position)) {
    *computed.get(&(depth, position)).unwrap()
  } else if depth == 0 || position == 0 {
    1
  } else {
    let mut sum = 0;
    for i in 0..=position {
      sum += calculate_paths_through_point(depth - 1, i, computed);
    }
    computed.insert((depth, position), sum);
    sum
  }
}

pub fn lattice_paths() -> u64 {
  calculate_paths_through_point(20, 20, &mut HashMap::new())
}

const NUMBER_LENGTH: [u64; 27] = [
  3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6, 8, 8, 7, 7, 9, 8, 8, 6, 6, 5, 5, 5, 7, 6,
  6,
];

#[inline(always)]
fn count_letters(num: u64) -> u64 {
  if num == 0 {
    return 0;
  }
  if num == 1000 {
    return 11;
  };
  let mut sum = 0;
  if num <= 20 {
    return NUMBER_LENGTH[(num - 1) as usize];
  } else if num < 100 {
    let rem = num % 10;
    let offset = ((num - rem) / 10) - 2;
    sum += count_letters(rem);
    sum += NUMBER_LENGTH[(19 + offset) as usize];
  } else {
    let rem = num % 100;
    let rem_count = count_letters(rem);
    sum += if rem_count > 0 { rem_count + 3 } else { 0 };
    sum += count_letters((num - rem) / 100);
    sum += 7;
  }
  sum
}

pub fn number_letter_counts() -> u64 {
  let mut sum = 0;
  for i in 1..=1000 {
    sum += count_letters(i);
  }
  sum
}

#[inline(always)]
fn max_sum_at_point(
  triangle: &Vec<Vec<u64>>,
  depth: usize,
  position: usize,
  memo: &mut HashMap<(u64, u64), u64>,
) -> u64 {
  let udepth = depth as u64;
  let upos = position as u64;
  if memo.contains_key(&(udepth, upos)) {
    return *memo.get(&(udepth, upos)).unwrap();
  };
  if depth == 0 {
    return triangle[0][0];
  }
  if position == 0 {
    let result = triangle[depth][0]
      + max_sum_at_point(triangle, depth - 1, position, memo);
    memo.insert((udepth, upos), result);
    return result;
  } else if position == triangle[depth].len() - 1 {
    let result = triangle[depth][position]
      + max_sum_at_point(triangle, depth - 1, position - 1, memo);
    memo.insert((udepth, upos), result);
    return result;
  }
  let result = triangle[depth][position]
    + u64::max(
      max_sum_at_point(triangle, depth - 1, position - 1, memo),
      max_sum_at_point(triangle, depth - 1, position, memo),
    );
  memo.insert((udepth, upos), result);
  result
}

#[inline(always)]
fn max_sum_at_depth(
  triangle: &Vec<Vec<u64>>,
  depth: usize,
  memo: &mut HashMap<(u64, u64), u64>,
) -> u64 {
  let mut largest = 0;
  for i in 0..triangle[depth].len() {
    largest = u64::max(largest, max_sum_at_point(triangle, depth, i, memo));
  }
  largest
}

pub fn maximum_path_sum_i() -> u64 {
  let triangle: Vec<Vec<u64>> = vec![
    vec![75],
    vec![95, 64],
    vec![17, 47, 82],
    vec![18, 35, 87, 10],
    vec![20, 04, 82, 47, 65],
    vec![19, 01, 23, 75, 03, 34],
    vec![88, 02, 77, 73, 07, 63, 67],
    vec![99, 65, 04, 28, 06, 16, 70, 92],
    vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
    vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
    vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
    vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
    vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
    vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
    vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
  ];
  let mut memo = HashMap::<(u64, u64), u64>::new();
  let result = max_sum_at_depth(&triangle, triangle.len() - 1, &mut memo);
  result
}

pub fn maximum_path_sum_ii() -> u64 {
  let triangle: Vec<Vec<u64>> = big_triangle::get();
  let mut memo = HashMap::<(u64, u64), u64>::new();
  max_sum_at_depth(&triangle, triangle.len() - 1, &mut memo)
}

const DAYS: [u8; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[derive(Debug, Clone, Copy)]
struct Calendar {
  weekday: u8,
  date: u8,
  month: u8,
  year: u16,
  leap_year_accounted: bool,
}

impl Calendar {
  fn new(weekday: u8, date: u8, month: u8, year: u16) -> Self {
    Self {
      weekday,
      date,
      month,
      year,
      leap_year_accounted: false,
    }
  }

  fn date_pair(&self) -> (u8, u8) {
    (self.weekday, self.date)
  }

  fn is_leap_year(&self) -> bool {
    let mod_four = self.year % 4 == 0;
    let mod_hundred = self.year % 100 == 0;
    let mod_four_hundred = self.year % 400 == 0;
    mod_four && (!mod_hundred || mod_four_hundred)
  }

  fn next_weekday(&mut self) {
    self.weekday = if self.weekday == 7 {
      1
    } else {
      self.weekday + 1
    };
    if self.date >= DAYS[self.month as usize] {
      if self.month == 2 && !self.leap_year_accounted && self.is_leap_year() {
        self.leap_year_accounted = true;
        self.date += 1;
        return;
      }
      self.leap_year_accounted = false;
      self.date = 1;
      if self.month == 12 {
        self.month = 1;
        self.year += 1;
      } else {
        self.month += 1
      }
    } else {
      self.date += 1
    }
  }
}

pub fn counting_sundays() -> u64 {
  let mut first_sundays = 0;
  let mut calendar = Calendar::new(2, 1, 1, 1900);
  while calendar.year < 2001 {
    if calendar.year >= 1901 && calendar.date_pair() == (1, 1) {
      first_sundays += 1
    };
    calendar.next_weekday();
  }
  first_sundays
}

#[inline(always)]
fn divisor_sum(num: u64) -> u64 {
  let mut sum = 1;
  for i in 2..=(num / 2) {
    if num % i == 0 {
      sum += i
    }
  }
  sum
}

#[inline(always)]
fn test_and_sum_amicable(
  num: u64,
  amicable_map: &mut HashMap<u64, u64>,
  amicable_sum: &mut u64,
) {
  let sum = divisor_sum(num);
  if sum == num {
    return;
  };
  if let Some(val) = amicable_map.get(&sum) {
    if *val == num {
      *amicable_sum += sum + num
    }
  } else {
    amicable_map.insert(num, sum);
  }
}

pub fn amicable_numbers() -> u64 {
  let mut amicable_map = HashMap::<u64, u64>::new();
  let mut amicable_sum = 0;
  for i in 1..10000 {
    test_and_sum_amicable(i, &mut amicable_map, &mut amicable_sum)
  }
  amicable_sum
}

const CHARS: [char; 26] = [
  'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
  'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[inline(always)]
fn alphabet_score(s: &str, vals: &HashMap<char, u8>) -> u64 {
  let mut score = 0;
  for char in s.chars() {
    score += *vals.get(&char).unwrap() as u64
  }
  score
}

pub fn names_scoares() -> u64 {
  let mut alphabet_vals = HashMap::<char, u8>::new();
  for i in 0..26 {
    alphabet_vals.insert(CHARS[i], i as u8 + 1);
  }
  let Ok(file) = File::open("./src/solutions/names.txt") else {
    panic!("Failed to open names file");
  };
  let line_reader = BufReader::new(file);
  let mut lines = line_reader.lines();
  let mut name_score_sum = 0;
  let mut names: Vec<String> = vec![];
  while let Some(Ok(line)) = lines.next() {
    names.push(line);
  }
  names.sort_unstable();
  for i in 0..names.len() {
    name_score_sum += alphabet_score(&names[i], &alphabet_vals) * (i as u64 + 1)
  }
  name_score_sum
}

#[inline(always)]
fn sum_factors(num: u64) -> u64 {
  let mut sum = 0;
  for i in 1..=(num / 2) {
    if num % i == 0 {
      sum += i
    };
  }
  sum
}

#[inline(always)]
fn is_abundant(num: u64) -> bool {
  sum_factors(num) > num
}

#[inline(always)]
fn has_abundant_constituents(num: u64, abundant_nums: &Vec<u64>) -> bool {
  let mut nums_iter = abundant_nums.iter();
  while let Some(some_num) = nums_iter.next() {
    let times_two = *some_num * 2;
    if times_two > num {
      break;
    };
    if times_two == num {
      return true;
    }
  }

  let mut lower = 0;
  let mut upper = abundant_nums.len() - 1;
  while lower < upper {
    let sum = abundant_nums[lower] + abundant_nums[upper];
    if sum == num {
      return true;
    } else if sum > num {
      upper -= 1;
    } else if sum < num {
      lower += 1;
    }
  }
  false
}

pub fn non_abundant_sums() -> u64 {
  let mut abundant_nums = Vec::<u64>::new();
  for i in 1..=28213 {
    if is_abundant(i) {
      abundant_nums.push(i);
    }
  }
  let mut total_sum = 28213 * 28214 / 2;
  for i in 1..=28213 {
    if has_abundant_constituents(i, &abundant_nums) {
      total_sum -= i;
    }
  }
  total_sum
}

trait Rotary {
  fn rotate_from(&mut self, idx: usize, times: u64);
}

impl Rotary for [u64] {
  #[inline(always)]
  fn rotate_from(&mut self, idx: usize, times: u64) {
    for _ in 0..times {
      for i in idx..self.len() {
        if self[i] > self[idx] {
          self.swap(i, idx);
          break;
        }
      }
    }
  }
}
trait Factorial {
  fn factorial(self) -> Self;
}

impl Factorial for u64 {
  #[inline(always)]
  fn factorial(mut self) -> Self {
    let mut f = 1;
    while self > 1 {
      f *= self;
      self -= 1;
    }
    f
  }
}

pub fn lexicographic_permutations() -> u64 {
  let mut digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  let mut permutations = 999999;
  let mut current_digit = 0;
  while permutations > 0 {
    let iterative_size =
      (((digits.len() - 1) - current_digit) as u64).factorial();
    let rotations = permutations / iterative_size;
    digits.rotate_from(current_digit, rotations);
    permutations -= iterative_size * rotations;
    current_digit += 1;
  }
  digits.into_iter().enumerate().fold(0, |acc, (idx, next)| {
    acc + (10u64.pow((digits.len() - (idx + 1)) as u32) * next)
  })
}
