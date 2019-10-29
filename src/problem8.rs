#![allow(dead_code)]
use std::cmp::max;

// The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
// Find the thirteen adjacent digits in the 1000-digit number that have the greatest product.
// What is the value of this product?
fn problem() -> u64 {
    let thousand_digit_number = "731671765313306249192251196744265747423553491949349698352031277450632623957831801698480186947885184385861560789112949495459501737958331952853208805511125406987471585238630507156932909632952274430435576689664895044524452316173185640309871112172238311362229893423380308135336276614282806444486645238749303589072962904915604407723907138105158593079608667017242712188399879790879227492190169972088809377665727333001053367881220235421809751254540594752243525849077116705560136048395864467063244157221553975369781797784617406495514929086256932197846862248283972241375657056057490261407972968652414535100474816637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

    thousand_digit_number.chars().enumerate()
        .filter_map(|(i, _)| thousand_digit_number.get(i..i + 13))
        .filter(|slice| !slice.chars().any(|c| c == '0'))
        .fold(0, |y, slice| {
            let x = slice.chars().fold(1, |acc, c| {
                acc * c.to_digit(10).unwrap() as u64
            });
            max(x, y)
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_test() {
        assert_eq!(problem(), 23514624000);
    }
}