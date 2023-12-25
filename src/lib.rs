pub mod factor;
pub mod fib;
pub mod primes;

pub fn binom(n: u64, k: u64) -> u64 {
    ((k + 1)..=n).reduce(|acc, e| e * acc).unwrap_or(1)
}
