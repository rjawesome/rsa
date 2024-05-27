use std::time::SystemTime;

use ibig::{ibig, modular::ModuloRing, ubig, IBig, UBig};
use num_traits::{One, Zero};
use rand::distributions::Uniform;
use ::rand::{thread_rng, Rng};

use crate::constants::PRIME_BITS;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let result = inverse(&ubig!(23), &ubig!(180));
        assert_eq!(result, ubig!(47));
    }
}

pub fn inverse(number: &UBig, modulus: &UBig) -> UBig {
    let mut ops: Vec<IBig> = Vec::new();
    let mut a = modulus.clone();
    let mut b = number.clone();
    while !b.is_one() {
        let div = &a / &b;
        let temp = b;
        b = a % &temp;
        a = temp;
        ops.push(IBig::from(div));
    }

    let mut c = ibig!(1);
    let mut d = ibig!(0);
    for div in ops.into_iter().rev() {
        let temp = c;
        c = d + -div * &temp;
        d = temp;
    }

    if c.signum() == ibig!(-1) {
        c += IBig::from(modulus);
    }

    UBig::try_from(c).unwrap()
}

pub fn gcd(a1: &UBig, b1: &UBig) -> UBig {
    let mut a = if a1 > b1 { a1.clone() } else { b1.clone() };
    let mut b = if b1 > a1 { a1.clone() } else { b1.clone() };
    while !b.is_zero() {
        let temp = b;
        b = a % &temp;
        a = temp;
    }
    a
}

pub fn get_prime() -> UBig {
    let mut count = 0;
    let mut rng = thread_rng();
    let dist = Uniform::new(ubig!(0), ubig!(1) << PRIME_BITS);
    loop {
        let unsigned: UBig = rng.sample(&dist);
        if is_prime(&unsigned) {
            return unsigned;
        }
        count += 1;
    }
}

// Miller-Rabin Test
fn is_prime(num: &UBig) -> bool {
    if !num.bit(num.bit_len() - 1) {
        return false;
    }

    let mut rng = thread_rng();
    let mut d: UBig = num - 1u32;
    let mut s = 0;
    while !d.bit(d.bit_len() - 1) {
        d >>= 1;
        s += 1;
    }

    let n_min_one = num - 1u32;
    let ring = ModuloRing::new(num);
    let two = ubig!(2);
    let one = ring.from(1);
    let min_one = ring.from(-1);
    let dist = Uniform::new(&two, &n_min_one);

    // operate in modular the whole time
    for _ in 0..5 {
        let a = rng.sample(&dist);
        let mut x = ring.from(a).pow(&d);
        for _ in 0..s {
            if x == one {
                break
            }
            let x_conditions = x != min_one;
            x = x.clone() * x;
            if x == one && x_conditions {
                return false;
            }
        }
        if x != one {
            return false;
        }
    }

    true
}