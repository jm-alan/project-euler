const factorial = (num) => {
  if (num < 3n) {
    return num;
  } else {
    let fact = 1n;
    while (num > 1n) {
      fact *= num;
      num -= 1n;
    }
    return fact;
  }
};

const sumDigits = (num) => {
  let sum = 0n;
  while (num > 0n) {
    const rem = num % 10n;
    sum += rem;
    num -= rem;
    num /= 10n;
  }
  return sum;
};

console.log(sumDigits(factorial(100n)));
