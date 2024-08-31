use itertools::Itertools;
use project_euler_solutions::factor::divisor_sum;

fn main() {
    let abundant_numbers = (1..=28123)
        .map(|num| {
            let sum_proper_divisors = divisor_sum(num) - num;
            (sum_proper_divisors, num)
        })
        .filter(|(sum_proper_divisors, num)| sum_proper_divisors > num)
        .map(|(_, num)| num as usize)
        .collect_vec();

    // println!("first coupla abundants: {:?}", &abundant_numbers[0..15]);

    // compute sums of abundants
    let mut is_idx_abundant_sum = [false; 28124];
    for (idx, lhs) in abundant_numbers.iter().enumerate() {
        for rhs in &abundant_numbers[idx..] {
            let val = lhs + rhs;
            if val > 28123 {
                break;
            }

            is_idx_abundant_sum[val] = true;
        }
    }

    // uncomment to debug

    // println!("is 24 a sum of abundants? {:?}", is_idx_abundant_sum[24]);
    // println!("is 23 a sum of abundants? {:?}", is_idx_abundant_sum[23]);
    // println!(
    //     "is 28123 a sum of abundants? {:?}",
    //     is_idx_abundant_sum[28123]
    // );
    // println!(
    //     "largest non-sum-of-abundants? {:?}",
    //     is_idx_abundant_sum
    //         .iter()
    //         .enumerate()
    //         .filter(|(_, is_sum_of_abundants)| !*is_sum_of_abundants)
    //         .map(|(num, _)| num)
    //         .last()
    // );  // 20161

    // find nums that are not sums of abundant nums
    println!(
        "{}",
        (1..=28123)
            .filter(|num| !is_idx_abundant_sum[*num])
            .sum::<usize>()
    );
}
