use itertools::Itertools;
use project_euler_solutions::factor::FactorIterator;

fn solution(n: u64) -> u64 {
    (1..=n)
        .flat_map(FactorIterator::new)
        .map(|f| (f.factor, f.power))
        .sorted()
        .into_iter()
        .group_by(|item| item.0)
        .into_iter()
        .map(|(key, group)| (key, group.map(|(_, v)| v).max().unwrap_or(0)))
        .map(|(base, power)| base.pow(power as u32))
        .fold(1, |acc, val| acc * val)
}

fn main() {
    println!("{}", solution(20))
}
