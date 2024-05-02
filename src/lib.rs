// port to rug

use std::fmt;
use std::str::FromStr;
use num_traits::pow;
use num_bigint::BigUint;
use std::error::Error;

mod utils;

pub struct PubKey {
    pub e: BigUint,
    pub n: BigUint
}

impl fmt::Display for PubKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", &self.e, &self.n)
    }
}

impl PubKey {
    pub fn new(pubtext: &str) -> Result<PubKey, &'static str> {
        let mut split = pubtext.split(",");
        let e_str = match split.next() {
            Some(x) => x,
            None => return Err("Invalid Public Key")
        };
        let e = BigUint::from_str(e_str).or_else(|_| Err("Invalid public key"))?;
        let n_str = match split.next() {
            Some(x) => x,
            None => return Err("Invalid public key")
        };
        let n = BigUint::from_str(n_str).or_else(|_| Err("Invalid public key"))?;
        Ok(PubKey {e, n})
    }
}

pub struct PrivKey {
    pub d: BigUint,
    pub n: BigUint
}

impl PrivKey {
    pub fn new(privtext: &str) -> Result<PrivKey, &'static str> {
        let mut split = privtext.split(",");
        let d_str = match split.next() {
            Some(x) => x,
            None => return Err("Invalid private key")
        };
        let d = BigUint::from_str(d_str).or_else(|_| Err("Invalid private key"))?;
        let n_str = match split.next() {
            Some(x) => x,
            None => return Err("Invalid private key")
        };
        let n = BigUint::from_str(n_str).or_else(|_| Err("Invalid private key"))?;
        Ok(PrivKey {d, n})
    }
}


impl fmt::Display for PrivKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", &self.d, &self.n)
    }
}

pub fn generate_keys() -> Result<(PubKey, PrivKey), Box<dyn Error>> {
    // generate variables
    let p = utils::get_prime();
    let pn1 = &p - 1u32;
    let q = utils::get_prime();
    let qn1 = &q - 1u32;
    let n = p * q;
    let lcm = &pn1 * &qn1 / utils::gcd(&pn1, &qn1);
    let e = BigUint::from(pow(2u32, 16) + 1);
    let d = utils::inverse(&e, &lcm);

    Ok((PubKey {e, n: n.clone()}, PrivKey {d, n}))
}

pub fn encode_text(plaintext: &[u8], pubkey: &PubKey) -> Result<Vec<u8>, Box<dyn Error>> {
    let plain_as_int = BigUint::from_bytes_be(plaintext);
    let encoded = plain_as_int.modpow(&pubkey.e, &pubkey.n); 
    Ok(encoded.to_bytes_be())
}

pub fn decode_text(ciphertext: &[u8], privkey: &PrivKey) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher_as_int = BigUint::from_bytes_be(ciphertext);
    let decoded = cipher_as_int.modpow(&privkey.d, &privkey.n); 
    Ok(decoded.to_bytes_be())
}