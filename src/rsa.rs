// port to rug

use std::fmt;
use std::str::FromStr;
use std::error::Error;

use ibig::modular::ModuloRing;
use ibig::UBig;

mod utils;

pub struct PubKey {
    pub e: UBig,
    pub n: UBig
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
        let e = UBig::from_str(e_str).or_else(|_| Err("Invalid public key"))?;
        let n_str = match split.next() {
            Some(x) => x,
            None => return Err("Invalid public key")
        };
        let n = UBig::from_str(n_str).or_else(|_| Err("Invalid public key"))?;
        Ok(PubKey {e, n})
    }
}

pub struct PrivKey {
    pub d: UBig,
    pub n: UBig
}

impl PrivKey {
    pub fn new(privtext: &str) -> Result<PrivKey, &'static str> {
        let mut split = privtext.split(",");
        let d_str = match split.next() {
            Some(x) => x,
            None => return Err("Invalid private key")
        };
        let d = UBig::from_str(d_str).or_else(|_| Err("Invalid private key"))?;
        let n_str = match split.next() {
            Some(x) => x,
            None => return Err("Invalid private key")
        };
        let n = UBig::from_str(n_str).or_else(|_| Err("Invalid private key"))?;
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
    let e = UBig::from((1u32 << 16) + 1);
    let d = utils::inverse(&e, &lcm);

    Ok((PubKey {e, n: n.clone()}, PrivKey {d, n}))
}

pub fn encode_text(plaintext: &[u8], pubkey: &PubKey) -> Result<Vec<u8>, Box<dyn Error>> {
    let plain_as_int = UBig::from_be_bytes(plaintext);
    let encoded = ModuloRing::new(&pubkey.n).from(plain_as_int).pow(&pubkey.e).residue();
    Ok(encoded.to_be_bytes())
}

pub fn decode_text(ciphertext: &[u8], privkey: &PrivKey) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher_as_int = UBig::from_be_bytes(ciphertext);
    let decoded = ModuloRing::new(&privkey.n).from(cipher_as_int).pow(&privkey.d).residue();
    Ok(decoded.to_be_bytes())
}