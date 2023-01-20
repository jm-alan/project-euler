const fibo = [1n, 1n, 2n];
const ceil = 10n ** 999n;

let idx = 3;

while (fibo[2] < ceil) {
  fibo[0] = fibo[1];
  fibo[1] = fibo[2];
  fibo[2] = fibo[0] + fibo[1];
  idx++;
}

console.log(idx);
