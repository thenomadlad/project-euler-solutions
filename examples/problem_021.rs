use std::collections::HashSet;

use project_euler_solutions::factor::divisor_sum;

fn main() {
    let mut amicable_numbers = HashSet::new();

    for value in 1..10_000 {
        if !amicable_numbers.contains(&value) {
            let div_sum = proper_divisor_sum(value);
            if proper_divisor_sum(div_sum) == value && div_sum != value {
                amicable_numbers.insert(div_sum);
                amicable_numbers.insert(value);
            }
        }
    }

    println!("{amicable_numbers:?}");
    println!("{}", amicable_numbers.iter().sum::<u64>());
}

fn proper_divisor_sum(num: u64) -> u64 {
    divisor_sum(num) - num
}
