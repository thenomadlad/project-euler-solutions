const POWER: usize = 1000;

fn main() {
    // println!("{}", solve(15.0));
    println!("{}", solve(POWER as f64));
}

fn solve(power: f64) -> usize {
    let max_digs = (power * f64::log(2.0, 10.0)).ceil() as usize;
    let mut digs = vec![0 as u8; max_digs];

    digs[0] = 1;
    for _ in 0..power as usize {
        let mut has_carry = false;
        for item in digs.iter_mut() {
            let val = *item * 2;
            *item = (val % 10) + if has_carry { 1 } else { 0 };
            has_carry = val >= 10;
        }
    }
    digs.iter().fold(0 as usize, |acc, v| acc + *v as usize)
}

// fn digsum(v: usize) -> usize {
//     let mut sum = 0;
//     let mut tempval = v;
//
//     while tempval > 0 {
//         sum += tempval % 10;
//         tempval /= 10;
//     }
//
//     sum
// }
