fn solution(n: u64) -> u64 {
    (1..=n)
        .flat_map(|k| (1..=n).map(move |j| (j, k)))
        .filter(|&(j, k)| j != k)
        .map(|(j, k)| j * k)
        .fold(0, |acc, num| acc + num)
}

fn main() {
    println!("{}", solution(100));
}
