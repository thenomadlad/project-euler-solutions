use project_euler_solutions::{bignum::BigNum, digsum, factorial};

fn main() {
    println!("{}! = {}", 100, soln(100));
}

fn soln(n: u64) -> u64 {
    (1..=n).fold(BigNum::one(), |acc, v| &acc * v).dig_sum()
}
