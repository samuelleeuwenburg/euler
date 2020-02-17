#![allow(dead_code)]

// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a^2 + b^2 = c^2
//
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
//
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.
fn euclids_formula(m: u64, n: u64) -> Option<(u64, u64, u64)> {
    if n >= m {
        None
    } else {
        let a = m.pow(2) - n.pow(2);
        let b = 2 * m * n;
        let c = m * m + n * n;

        Some((a, b, c))
    }
}

fn problem(target_sum: u64) -> u64 {
    let mut m = 2;
    let result: u64;

    'main: loop {
        let upper_n = m;

        for n in 0..upper_n {
            let (a, b, c) = euclids_formula(m, n).unwrap();
            let sum = a + b + c;
           
            if sum == target_sum {
                result = a * b * c;
                break 'main;
            }
        }

        m = m + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclids_formula_test() {
        assert_eq!(euclids_formula(1, 2), None);
        assert_eq!(euclids_formula(2, 5), None);
        assert_eq!(euclids_formula(3, 3), None);

        assert_eq!(euclids_formula(2, 1), Some((3, 4, 5)));
        assert_eq!(euclids_formula(4, 1), Some((15, 8, 17)));
        assert_eq!(euclids_formula(8, 7), Some((15, 112, 113)));
        assert_eq!(euclids_formula(10, 5), Some((75, 100, 125)));
    }

    #[test]
    fn problem_test() {
        // m = 20, n = 5 will result in the solution
        assert_eq!(problem(1000), 375 * 200 * 425);
    }
}
