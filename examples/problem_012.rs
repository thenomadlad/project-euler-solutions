use project_euler_solutions::factor::FactorIterator;

struct TriangleNumbers {
    current: u64,
    index: u64,
}

impl TriangleNumbers {
    pub fn new() -> TriangleNumbers {
        TriangleNumbers {
            current: 0,
            index: 0,
        }
    }
}

impl Iterator for TriangleNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.current += self.index;
        Some(self.current)
    }
}

fn get_num_factors(value: u64) -> u64 {
    FactorIterator::new(value)
        .map(|ftrn| ftrn.power as u64 + 1) // number of ways the factor can be powered - 0 to n
        .product() // combination of all of them
}

fn main() {
    let result = TriangleNumbers::new()
        .find(|&value| get_num_factors(value) > 500)
        .unwrap();

    println!("{}", result);
}
