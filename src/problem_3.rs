use crate::util::factor::FactorIterator;

pub fn largest_prime_factor(num: u64) -> u64 {
    FactorIterator::new(num).map(|x| x.factor).max().unwrap_or(1)
}
