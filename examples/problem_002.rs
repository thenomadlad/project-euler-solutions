use project_euler_solutions::fib::Fibs;

fn main() {
    let result: u64 = Fibs::default()
        .take_while(|x| *x <= 4_000_000)
        .filter(|x| x % 2 == 0)
        .sum();

    println!("{}", result);
}
