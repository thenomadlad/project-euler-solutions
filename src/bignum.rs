use core::ops::Add;
use std::{cmp::Ordering, ops::Mul};

use itertools::Itertools;

const BASE: u64 = 1_000_000_000;

#[derive(PartialEq, Debug, PartialOrd)]
pub struct BigNum {
    digs: Vec<u64>,
}

impl BigNum {
    pub fn new() -> Self {
        Self::new_with_digs(vec![0])
    }

    pub fn one() -> Self {
        Self::new_with_digs(vec![1])
    }

    pub fn new_with_digs(digs: Vec<u64>) -> Self {
        BigNum { digs }
    }

    pub fn dig_sum(&self) -> u64 {
        self.digs
            .iter()
            .map(|&val| {
                val.to_string()
                    .chars()
                    .map(|ch| ch as u64 - '0' as u64)
                    .sum::<u64>()
            })
            .sum()
    }
}

impl<'a, 'b> Add<&'a BigNum> for &'b BigNum {
    type Output = BigNum;

    fn add(self, rhs: &'a BigNum) -> Self::Output {
        let mut result = vec![];
        let mut carry = false;
        for item in Itertools::zip_longest(self.digs.iter(), rhs.digs.iter()) {
            let value = match item {
                itertools::EitherOrBoth::Both(left, right) => left + right,
                itertools::EitherOrBoth::Left(x) => *x,
                itertools::EitherOrBoth::Right(x) => *x,
            } + if carry { 1 } else { 0 };

            // append to vec
            result.push(value % BASE);
            if value >= BASE {
                carry = true;
            } else {
                carry = false;
            }
        }

        if carry {
            result.push(1);
        }

        BigNum::new_with_digs(result)
    }
}

impl Mul<u64> for &BigNum {
    type Output = BigNum;

    fn mul(self, rhs: u64) -> Self::Output {
        let mut result = vec![];
        let mut carry = 0;
        for item in self.digs.iter() {
            let value = item * rhs + carry;
            result.push(value % BASE);
            carry = value / BASE;
        }

        if carry > 0 {
            result.push(carry);
        }

        let last_non_zero_idx = result
            .iter()
            .enumerate()
            .filter(|(_, val)| **val > 0)
            .map(|(idx, _)| idx)
            .max();

        result = result
            .into_iter()
            .take(last_non_zero_idx.unwrap_or(0) + 1)
            .collect();

        BigNum::new_with_digs(result)
    }
}

impl<'a, 'b> Mul<&'a BigNum> for &'b BigNum {
    type Output = BigNum;

    fn mul(self, rhs: &'a BigNum) -> Self::Output {
        // use the multiplication algorithm here
        let (smaller, greater) = match self.partial_cmp(rhs) {
            Some(Ordering::Less) | Some(Ordering::Equal) | None => (self, rhs),
            Some(Ordering::Greater) => (rhs, self),
        };

        smaller
            .digs
            .iter()
            .enumerate()
            .map(|(idx, &val)| {
                let offset = vec![0; idx];
                let multiplied = greater * val;

                BigNum::new_with_digs(
                    offset
                        .into_iter()
                        .chain(multiplied.digs.into_iter())
                        .collect(),
                )
            })
            .fold(BigNum::new(), |acc, val| &acc + &val)
    }
}

#[cfg(test)]
mod tests {
    use crate::bignum::BigNum;

    #[test]
    fn adds_base_cases() {
        assert_eq!(&BigNum::new() + &BigNum::new(), BigNum::new())
    }

    #[test]
    fn adds_empty_lhs_rhs() {
        let val = BigNum::new_with_digs(vec![999999, 9]);
        assert_eq!(&val + &BigNum::new(), val);
        assert_eq!(&BigNum::new() + &val, val);
    }

    #[test]
    fn adds_two_big_nums() {
        let left = BigNum::new_with_digs(vec![999999999, 9]);
        let right = BigNum::new_with_digs(vec![1, 1]);

        assert_eq!(&left + &right, BigNum::new_with_digs(vec![0, 11]));
    }

    #[test]
    fn add_carry_beyond_num_size() {
        let left = BigNum::new_with_digs(vec![999999999]);
        let right = BigNum::new_with_digs(vec![1]);

        assert_eq!(&left + &right, BigNum::new_with_digs(vec![0, 1]));
    }

    #[test]
    fn mul_num_base_cases() {
        assert_eq!(
            &BigNum::new_with_digs(vec![1]) * 1,
            BigNum::new_with_digs(vec![1])
        )
    }

    #[test]
    fn mul_num_with_zero() {
        let val = BigNum::new_with_digs(vec![999999, 9]);
        assert_eq!(&val * 0, BigNum::new());
    }

    #[test]
    fn mul_num_with_empty_bignum() {
        let val = BigNum::new();
        assert_eq!(&val * 7, BigNum::new());
    }

    #[test]
    fn mul_num_with_big_num() {
        let left = BigNum::new_with_digs(vec![112233, 1]);

        assert_eq!(&left * 2, BigNum::new_with_digs(vec![224466, 2]));
    }

    #[test]
    fn mul_num_with_big_num_with_carry() {
        let left = BigNum::new_with_digs(vec![555_555_555, 1]);

        assert_eq!(&left * 2, BigNum::new_with_digs(vec![111_111_110, 3]));
    }

    #[test]
    fn mul_num_carry_beyond_num_size() {
        let left = BigNum::new_with_digs(vec![555_555_555]);

        assert_eq!(&left * 2, BigNum::new_with_digs(vec![111_111_110, 1]));
    }

    #[test]
    fn mul_base_cases() {
        assert_eq!(
            &BigNum::new_with_digs(vec![1]) * &BigNum::new_with_digs(vec![1]),
            BigNum::new_with_digs(vec![1])
        )
    }

    #[test]
    fn mul_with_zero() {
        let val = BigNum::new_with_digs(vec![999999, 9]);
        assert_eq!(&val * &BigNum::new(), BigNum::new());
        assert_eq!(&BigNum::new() * &val, BigNum::new());
    }

    #[test]
    fn mul_with_big_num() {
        let left = BigNum::new_with_digs(vec![112233, 1]);
        let right = BigNum::new_with_digs(vec![2]);

        assert_eq!(&left * &right, BigNum::new_with_digs(vec![224466, 2]));
    }

    #[test]
    fn mul_with_big_num_with_carry() {
        let left = BigNum::new_with_digs(vec![555_555_555, 1]);
        let right = BigNum::new_with_digs(vec![2]);

        assert_eq!(&left * &right, BigNum::new_with_digs(vec![111_111_110, 3]));
    }

    #[test]
    fn mul_with_large_big_num_with_carry() {
        let left = BigNum::new_with_digs(vec![555_555_555, 1]);
        let right = BigNum::new_with_digs(vec![2, 2]);

        assert_eq!(
            &left * &right,
            BigNum::new_with_digs(vec![111_111_110, 111_111_113, 3])
        );
    }

    #[test]
    fn mul_carry_beyond_num_size() {
        let left = BigNum::new_with_digs(vec![555_555_555]);
        let right = BigNum::new_with_digs(vec![2]);

        assert_eq!(&left * &right, BigNum::new_with_digs(vec![111_111_110, 1]));
    }

    #[test]
    fn mul_carry_large_big_num_beyond_num_size() {
        let left = BigNum::new_with_digs(vec![555_555_555]);
        let right = BigNum::new_with_digs(vec![0, 2]);

        assert_eq!(
            &left * &right,
            BigNum::new_with_digs(vec![0, 111_111_110, 1])
        );
    }

    #[test]
    fn digsum() {
        assert_eq!(BigNum::new().dig_sum(), 0);
        assert_eq!(BigNum::new_with_digs(vec![22]).dig_sum(), 4);
        assert_eq!(BigNum::new_with_digs(vec![22, 1]).dig_sum(), 5);
        assert_eq!(
            BigNum::new_with_digs(vec![999_999_999, 999_999_999]).dig_sum(),
            162
        );
    }
}
