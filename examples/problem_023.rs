use std::collections::HashSet;

use project_euler_solutions::factor::divisor_sum;

fn main() {
    let mut abundant_numbers = HashSet::new();

    // find sufficient list of abundant numbers
    for num in 1..=28123 {
        let sum_proper_divisors = divisor_sum(num) - num;

        if sum_proper_divisors > num {
            abundant_numbers.insert(num);
        }
    }

    // find nums that are not sums of abundant nums
    println!(
        "{}",
        (0..28123)
            .filter(|num| !is_sum_abundant_nums(*num, &abundant_numbers))
            .sum::<u64>()
    );
}

fn is_sum_abundant_nums(num: u64, abundant_numbers: &HashSet<u64>) -> bool {
    for abundant_num in abundant_numbers.iter() {
        if (*abundant_num < num) && abundant_numbers.contains(&(num - abundant_num)) {
            return true;
        }
    }

    false
}
