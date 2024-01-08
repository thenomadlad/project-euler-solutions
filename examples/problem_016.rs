use project_euler_solutions::bignum::BigNum;

const POWER: usize = 1000;

fn main() {
    println!("{}", solve(POWER));
}

fn solve(power: usize) -> u64 {
    (0..power).fold(BigNum::one(), |acc, _| &acc * 2).dig_sum()
}
