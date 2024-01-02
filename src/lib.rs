pub mod factor;
pub mod fib;
pub mod primes;
pub mod triangle;

pub fn binom(n: u64, k: u64) -> u64 {
    ((k + 1)..=n).reduce(|acc, e| e * acc).unwrap_or(1)
}

pub fn factorial(n: u64) -> u64 {
    (1..=n).fold(1, |acc, v| acc * v)
}

pub fn digsum(n: u64) -> u64 {
    let mut n = n;
    let mut sum = 0;
    while n > 0 {
        let dig = n % 10;
        n = n / 10;
        sum += dig;
    }

    sum
}
