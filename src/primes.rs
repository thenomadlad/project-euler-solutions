pub struct PrimesIterator {
    primes_so_far: Vec<u64>,
    current: u64,
}

impl Default for PrimesIterator {
    fn default() -> Self {
        Self {
            primes_so_far: vec![],
            current: 2,
        }
    }
}

impl Iterator for PrimesIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.primes_so_far.iter().any(|x| self.current % x == 0) {
            self.current += 1
        }

        self.primes_so_far.push(self.current);
        Some(self.current)
    }
}

pub fn sieve(n: usize) -> Vec<u64> {
    let mut data = (2..n as u64).collect::<Vec<_>>();
    let mut idx = 0;

    while idx < data.len() {
        let cur_prime = data[idx];
        if cur_prime != 0 {
            let mut cur_index = idx + cur_prime as usize;
            while cur_index < data.len() {
                data[cur_index] = 0;
                cur_index += cur_prime as usize;
            }
        }

        idx += 1;
    }

    data.into_iter().filter(|&v| v > 0).collect()
}
