pub struct FactorIterator {
    remaining_num: u64,
    current_prime: u64,
}

impl FactorIterator {
    pub fn new(start: u64) -> Self {
        FactorIterator {
            remaining_num: start, // we need to find all the factors of this number
            current_prime: 2,
        }
    }
}

pub struct Factorization {
    pub factor: u64,
    pub power: u64,
}

impl Iterator for FactorIterator {
    type Item = Factorization;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_num <= 1 {
            None
        } else {
            while self.remaining_num % self.current_prime != 0 {
                self.current_prime += 1;
            }

            let mut power = 0;
            while self.remaining_num % self.current_prime == 0 {
                power += 1;
                self.remaining_num /= self.current_prime;
            }

            Some(Factorization {
                factor: self.current_prime,
                power,
            })
        }
    }
}