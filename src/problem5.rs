#![allow(dead_code)]
use std::collections::HashMap;
use std::cmp::max;
use crate::prime::{Primes};

// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.  
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
fn problem(num: usize) -> u64 {
    let mut primes = Primes::new();
    let mut factor_map: HashMap<u64, u64> = HashMap::new();

    for n in 2..(num + 1) {
        let n = n as u64;
        if primes.is_prime(n) {
            let total = factor_map.get(&n).copied().unwrap_or(1);
            factor_map.insert(n, total);
            continue;
        }

        let factors = primes.get_prime_factorization(n).into_iter();

        for p in factor_map.clone().keys() {
            let x = factor_map.get(p).copied().unwrap_or(0);
            let y = factors.clone().fold(0, |ys, y| {
                if y == *p { ys + 1 } else { ys }
            });

            factor_map.insert(*p, max(x, y));
        }
    }

    factor_map.iter().fold(1, |acc, (k, &v)| {
        acc * k.pow(v as u32)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_test() {
        assert_eq!(problem(10), 2520);
        assert_eq!(problem(20), 232_792_560);
        assert_eq!(problem(42), 219_060_189_739_591_200);
    }
}
