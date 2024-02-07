fn solution(n: u64) -> u64 {
    (1..=n)
        .flat_map(|k| (1..=n).map(move |j| (j, k)))
        .filter(|&(j, k)| j != k)
        .map(|(j, k)| j * k)
        .sum::<u64>()
}

fn main() {
    println!("{}", solution(100));
}
