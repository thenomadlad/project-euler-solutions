use std::collections::HashMap;

fn compute_collatz_number(start: u64, cache: &mut HashMap<u64, usize>) -> usize {
    cache.get(&start).map(|v| *v).unwrap_or_else(|| {
        let value = if start == 1 {
            1
        } else {
            if start % 2 == 0 {
                1 + compute_collatz_number(start / 2, cache)
            } else {
                1 + compute_collatz_number((3 * start) + 1, cache)
            }
        };

        cache.insert(start, value);
        value
    })
}

fn main() {
    let mut max_start = 0;
    let mut max_collatz_number = 0;

    let mut cache = HashMap::new();

    for start in 1..1_000_000 {
        let collatz_number = compute_collatz_number(start, &mut cache);

        if collatz_number >= max_collatz_number {
            max_collatz_number = collatz_number;
            max_start = start;
        }
    }

    println!("{max_start}: {max_collatz_number}");
}
