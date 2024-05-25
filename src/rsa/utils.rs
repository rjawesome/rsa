use num_bigint::Sign;
use rand::distributions::Uniform;
use rand::Rng;
use num_bigint::{BigInt, BigUint, RandomBits};
use num_traits::{Zero, One};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let result = inverse(&BigUint::from(23u32), &BigUint::from(180u32));
        assert_eq!(result, BigUint::from(47u32));
    }
}

pub fn inverse(number: &BigUint, modulus: &BigUint) -> BigUint {
    let mut ops: Vec<BigInt> = Vec::new();
    let mut a = modulus.clone();
    let mut b = number.clone();
    while !One::is_one(&b) {
        let div = &a / &b;
        let temp = b;
        b = &a % &temp;
        a = temp;
        ops.push(BigInt::from_biguint(Sign::Plus, div));
    }

    let mut c: BigInt = One::one();
    let mut d: BigInt = Zero::zero();
    for div in ops.iter().rev() {
        let temp = c;
        c = d + &temp * -div;
        d = temp;
    }

    if c.sign() == Sign::Minus {
        c += BigInt::from_biguint(Sign::Plus, modulus.clone());
    }

    c.to_biguint().unwrap()
}

pub fn gcd(a1: &BigUint, b1: &BigUint) -> BigUint {
    let mut a = if a1 > b1 { a1.clone() } else { b1.clone() };
    let mut b = if b1 > a1 { a1.clone() } else { b1.clone() };
    while !Zero::is_zero(&b) {
        let temp = b;
        b = &a % &temp;
        a = temp;
    }
    a
}

pub fn get_prime() -> BigUint {
    let mut rng = rand::thread_rng();
    loop {
        let unsigned: BigUint = rng.sample(RandomBits::new(1024));
        if is_prime(&unsigned) {
            return unsigned;
        }
    }
}

// num should be odd
fn is_prime(num: &BigUint) -> bool {
    if num % 2u32 == Zero::zero() {
        return false;
    }

    let mut rng = rand::thread_rng();
    let mut d: BigUint = num - 1u32;
    let mut s = 0;
    while &d % 2u32 == Zero::zero() {
        d /= 2u32;
        s += 1;
    }

    for _ in 0..10 {
        let a = rng.sample(Uniform::new(BigUint::from(2u32), num - 2u32));
        let mut x = a.modpow(&d, num);
        for _ in 0..s {
            let y = x.modpow(&BigUint::from(2u32), num);
            if y == One::one() && x != One::one() && x != num - 1u32 {
                return false;
            }
            x = y;
        }
        if x != One::one() {
            return false;
        }
    }

    true
}