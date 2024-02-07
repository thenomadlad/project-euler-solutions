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

#[derive(Debug)]
pub struct Factorization {
    pub factor: u64,
    pub power: u32,
}

impl Iterator for FactorIterator {
    type Item = Factorization;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_num <= 1 {
            None
        } else {
            while self.remaining_num % self.current_prime != 0 {
                // if current_prime is 2, bump up to 3, else skip the even numbers
                self.current_prime += if self.current_prime == 2 { 1 } else { 2 };
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

pub fn divisor_sum(num: u64) -> u64 {
    if num > 0 {
        FactorIterator::new(num)
            .map(|ftrzn| (ftrzn.factor.pow(ftrzn.power + 1) - 1) / (ftrzn.factor - 1))
            .product::<u64>()
    } else {
        // rust's iterator.product() method returns 1 if the list of factors is empty. So in the
        // cases of num=0, when we get an empty list of factors, the above block returns an
        // incorrect divisor_sum of 1
        0
    }
}
