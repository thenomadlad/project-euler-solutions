pub mod bignum;
pub mod factor;
pub mod fib;
pub mod primes;
pub mod triangle;

pub fn combinations(n: u64, k: u64) -> u64 {
    ((k + 1)..=n).reduce(|acc, e| e * acc).unwrap_or(1)
}

pub fn factorial(n: u64) -> u64 {
    (1..=n).product::<u64>()
}

pub fn digsum(n: u64) -> u64 {
    let mut n = n;
    let mut sum = 0;
    while n > 0 {
        let dig = n % 10;
        n /= 10;
        sum += dig;
    }

    sum
}
