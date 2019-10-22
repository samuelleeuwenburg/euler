// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?
pub fn _problem_3() {
}

fn _is_prime(num: usize) -> bool {
    let round_division = (2..num)
        .into_iter()
        .find(|x| (num as f32 / *x as f32).fract() == 0f32);
       
    match round_division {
        Some(_) => false,
        None => true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_works() {
        assert_eq!(_is_prime(2), true);
        assert_eq!(_is_prime(3), true);
        assert_eq!(_is_prime(5), true);
        assert_eq!(_is_prime(7), true);
        assert_eq!(_is_prime(13), true);
        assert_eq!(_is_prime(19), true);
        assert_eq!(_is_prime(29), true);
        assert_eq!(_is_prime(97), true);

        assert_eq!(_is_prime(4), false);
        assert_eq!(_is_prime(6), false);
        assert_eq!(_is_prime(16), false);
        assert_eq!(_is_prime(21), false);
        assert_eq!(_is_prime(30), false);
        assert_eq!(_is_prime(98), false);
    }
}
