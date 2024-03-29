use project_euler_solutions::primes;

fn main() {
    println!("{}", solution(2_000_000));
}

fn solution(n: usize) -> u64 {
    primes::sieve(n).iter().sum::<u64>()
}
