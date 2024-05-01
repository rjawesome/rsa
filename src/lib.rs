// port to rug

use std::env;
use std::fs;
use std::str::FromStr;
use num_traits::pow;
use num_bigint::BigUint;
use std::error::Error;
use args::Type;

mod utils;
mod args;
mod errors;

pub fn run() {
    let mut all_args = env::args();
    let op_type = args::get_type(&mut all_args).unwrap_or_else(|err| {
        panic!("Error in Arguments: {}", err);
    });

    let result = match op_type {
        Type::GenKeys => generate_keys(all_args),
        Type::Decode => decode_text(all_args),
        Type::Encode => encode_text(all_args)
    };

    result.unwrap_or_else(|err| {
        panic!("Error: {}", err.to_string());
    });
}

fn generate_keys(os_args: env::Args) -> Result<(), Box<dyn Error>> {
    let args = args::GenArgs::new(os_args)?;

    // generate variables
    let p = utils::get_prime();
    let pn1 = &p - 1u32;
    let q = utils::get_prime();
    let qn1 = &q - 1u32;
    let n = p * q;
    let lcm = &pn1 * &qn1 / utils::gcd(&pn1, &qn1);
    let e = BigUint::from(pow(2u32, 16) + 1);
    let d = utils::inverse(&e, &lcm);

    // fmt keys
    let pubkey = format!("{},{}", &e, &n);
    let privkey = format!("{},{}", &d, &n);
    fs::write(&args.pub_filename, pubkey)?;
    fs::write(&args.priv_filename, privkey)?;

    Ok(())
}

fn encode_text(os_args: env::Args) -> Result<(), Box<dyn Error>> {
    let args = args::EncArgs::new(os_args)?;

    // get variables
    let pubtext = fs::read_to_string(args.pub_filename)?;
    let mut split = pubtext.split(",");
    let plaintext = fs::read(args.plaintext_filename)?;
    let e_str = match split.next() {
        Some(x) => x,
        None => return Err(Box::new(errors::InvalidFileError))
    };
    let e = BigUint::from_str(e_str).or_else(|_| Err(Box::new(errors::InvalidFileError)))?;
    let n_str = match split.next() {
        Some(x) => x,
        None => return Err(Box::new(errors::InvalidFileError))
    };
    let n = BigUint::from_str(n_str).or_else(|_| Err(Box::new(errors::InvalidFileError)))?;

    // encoding
    let plain_as_int = BigUint::from_bytes_be(&plaintext);
    let encoded = plain_as_int.modpow(&e, &n); 
    fs::write(args.ciphertext_filename, encoded.to_bytes_be())?;

    Ok(())
}

fn decode_text(os_args: env::Args) -> Result<(), Box<dyn Error>> {
    let args = args::DecArgs::new(os_args)?;

    // get variables
    let privtext = fs::read_to_string(args.priv_filename)?;
    let mut split = privtext.split(",");
    let ciphertext = fs::read(args.ciphertext_filename)?;
    let d_str = match split.next() {
        Some(x) => x,
        None => return Err(Box::new(errors::InvalidFileError))
    };
    let d = BigUint::from_str(d_str).or_else(|_| Err(Box::new(errors::InvalidFileError)))?;
    let n_str = match split.next() {
        Some(x) => x,
        None => return Err(Box::new(errors::InvalidFileError))
    };
    let n = BigUint::from_str(n_str).or_else(|_| Err(Box::new(errors::InvalidFileError)))?;

    // encoding
    let cipher_as_int = BigUint::from_bytes_be(&ciphertext);
    let decoded = cipher_as_int.modpow(&d, &n); 
    fs::write(args.plaintext_filename, decoded.to_bytes_be())?;

    Ok(())
}