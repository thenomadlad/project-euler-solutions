fn main() {
    for k in 3..=1000 {
        for i in 1..=(k / 2) {
            let j: u32 = (1000 - i) - k;

            if i.pow(2) + j.pow(2) == k.pow(2) {
                println!("{}^2 + {}^2 = {}^2", i, j, k);
                println!("{}", i * j * k);
                return;
            }
        }
    }
}
