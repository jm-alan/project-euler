pub fn mult_three_five() -> u64 {
  let mut sum: u64 = 0;
  for i in 0..1000 {
    if i % 3 == 0 || i % 5 == 0 {
      sum += i
    }
  }
  sum
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
  let mut c = (a.powf(2.0) + b.powf(2.0)).sqrt();
  while a < 400.0 {
    while b < 400.0 {
      let a2 = a.powf(2.0);
      let b2 = b.powf(2.0);
      let interstitial_pow = a2 + b2;
      c = (interstitial_pow as f64).sqrt();
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
