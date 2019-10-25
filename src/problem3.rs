#![allow(dead_code)]
use std::collections::HashMap;

// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?
fn problem(num: u64) -> u64 {
    let prime_factors = get_prime_factorization(num);
    prime_factors.last().cloned().unwrap()
}

// @TODO: add to Primes impl
pub fn get_prime_factorization(num: u64) -> Vec<u64> {
    let primes = Primes::new();

    fn rec(mut primes: Primes, mut factors: Vec<u64>, x: u64) -> Vec<u64> {
        if primes.is_prime(x) {
            factors.push(x);
            return factors
        }

        let factor = (2..x)
            .find_map(|y| {
                if !primes.is_prime(y) {
                    return None
                }

                let is_round_division = (x as f64 / y as f64).fract() == 0f64;

                if !is_round_division {
                    return None
                }

                Some((x / y, y))
            });

        match factor {
            Some((remainder, factor)) => {
                factors.push(factor);
                rec(primes, factors, remainder)
            }
            None => {
                factors 
            }
        }
    }

    rec(primes, vec![], num)
}

// @TODO: move to prime file
pub struct Primes {
    found_primes: HashMap<u64, bool>,
}

impl Primes {
    pub fn new() -> Primes {
        Primes { found_primes: HashMap::new() }
    }

    pub fn is_prime(&mut self, num: u64) -> bool {
        match self.found_primes.get(&num) {
            Some(&is_prime) => is_prime,
            None => {
                let round_division = (2..num)
                    .into_iter()
                    .find(|&x| (num as f64 / x as f64).fract() == 0f64);

                let is_prime = match round_division {
                    Some(_) => false,
                    None => true
                };

                self.found_primes.insert(num, is_prime);
                is_prime
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_works() {
        assert_eq!(problem(12), 3);
        assert_eq!(problem(42), 7);
        assert_eq!(problem(128), 2);
        assert_eq!(problem(60100), 601);
        assert_eq!(problem(600_851_475_143), 6857);
    }

    #[test]
    fn get_prime_factorization_works() {
        assert_eq!(get_prime_factorization(9), [3, 3]);
        assert_eq!(get_prime_factorization(128), [2, 2, 2, 2, 2, 2, 2]);
        assert_eq!(get_prime_factorization(42), [2, 3, 7]);
        assert_eq!(get_prime_factorization(60100), [2, 2, 5, 5, 601]);
        assert_eq!(get_prime_factorization(600_851_475_143), [71, 839, 1471, 6857]);
    }

    #[test]
    fn is_prime_works() {
        let mut primes = Primes::new();

        assert_eq!(primes.is_prime(2), true);
        assert_eq!(primes.is_prime(3), true);
        assert_eq!(primes.is_prime(29), true);
        assert_eq!(primes.is_prime(97), true);

        assert_eq!(primes.is_prime(4), false);
        assert_eq!(primes.is_prime(21), false);
        assert_eq!(primes.is_prime(42), false);
    }
}
