use std::collections::HashMap;

// Sieve of Eratosthenes
pub fn find_primes_up_to(num: u64) -> Vec<u64> {
    fn rec(remaining_nums: &Vec<u64>, p: u64) -> Vec<u64> {
        remaining_nums
            .into_iter()
            .find(|&&n| n != p && (n as f64 / p as f64).fract() == 0.0)
            .map(|_| {
                let ns: Vec<u64> = remaining_nums
                    .iter()
                    .copied()
                    .filter(|&n| n == p || (n as f64 / p as f64).fract() != 0.0)
                    .collect();

                let next_prime = remaining_nums
                    .into_iter()
                    .find(|&&n| n > p)
                    .unwrap();

                rec(&ns, *next_prime)
            })
            .unwrap_or(remaining_nums.iter().copied().collect())
    }

    let nums: Vec<u64> = (2..num).collect();
    rec(&nums, 2)
}

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
                let square_root_rounded = (num as f64).sqrt().floor() + 1.0;
                let round_division = (2..square_root_rounded as u64)
                    .into_iter()
                    .find(|&x| {
                        (num as f64 / x as f64).fract() == 0f64
                    });

                let is_prime = match round_division {
                    Some(_) => false,
                    None => true
                };

                self.found_primes.insert(num, is_prime);
                is_prime
            }
        }
    }
  
    pub fn get_prime_factorization(&mut self, num: u64) -> Vec<u64> {
        fn rec(primes: &mut Primes, mut factors: Vec<u64>, x: u64) -> Vec<u64> {
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

        rec(self, vec![], num)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime() {
        let mut primes = Primes::new();

        assert_eq!(primes.is_prime(2), true);
        assert_eq!(primes.is_prime(3), true);
        assert_eq!(primes.is_prime(29), true);
        assert_eq!(primes.is_prime(97), true);

        assert_eq!(primes.is_prime(4), false);
        assert_eq!(primes.is_prime(21), false);
        assert_eq!(primes.is_prime(42), false);
    }

    #[test]
    fn find_primes_up_to_test() {
        assert_eq!(find_primes_up_to(11), vec![2, 3, 5, 7]);
        assert_eq!(find_primes_up_to(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn get_prime_factorization() {
        let mut primes = Primes::new();

        assert_eq!(primes.get_prime_factorization(9), [3, 3]);
        assert_eq!(primes.get_prime_factorization(128), [2, 2, 2, 2, 2, 2, 2]);
        assert_eq!(primes.get_prime_factorization(42), [2, 3, 7]);
        assert_eq!(primes.get_prime_factorization(60100), [2, 2, 5, 5, 601]);
    }
}
