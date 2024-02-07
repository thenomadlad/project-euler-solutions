use project_euler_solutions::primes::PrimesIterator;

fn solution(n: usize) -> u64 {
    PrimesIterator::default().take(n).last().unwrap()
}

fn main() {
    println!("{}", solution(10001))
}
