use project_euler_solutions::factor::FactorIterator;

fn main() {
    println!(
        "{}",
        FactorIterator::new(600851475143)
            .map(|x| x.factor)
            .max()
            .unwrap_or(1)
    );
}
