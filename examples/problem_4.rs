fn is_palindrome(num: u64) -> bool {
    (num / 100000 == num % 10)
        && ((num / 10000) % 10 == (num % 100) / 10)
        && ((num / 1000) % 10 == (num % 1000) / 100)
}

fn main() {
    let (x, y, product) = (100..1000)
        .rev()
        .flat_map(|x| (100..1000).rev().map(move |y| (x, y)))
        .map(|(factor1, factor2)| (factor1, factor2, factor1 * factor2))
        .filter(|&(_, _, product)| is_palindrome(product))
        .max_by_key(|&(_, _, product)| product)
        .unwrap();

    println!("{} x {} = {}", x, y, product)
}
