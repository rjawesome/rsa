mod args;
use std::{env, error::Error, fs};
use rsa_tool::rsa;
use args::{DecArgs, EncArgs, GenArgs, Type};

fn main() -> Result<(), Box<dyn Error>> {
    let mut all_args = env::args();
    let op_type = args::get_type(&mut all_args).unwrap_or_else(|err| {
        panic!("Error in Arguments: {}", err);
    });

    match op_type {
        Type::GenKeys => {
            let gen_args = GenArgs::new(all_args)?;
            let (pubkey, privkey) = rsa::generate_keys()?;
            fs::write(&gen_args.pub_filename, pubkey.to_string()).or_else(|_| Err("Error writing to public key file"))?;
            fs::write(&gen_args.priv_filename, privkey.to_string()).or_else(|_| Err("Error writing to private key file"))?;
        },
        Type::Decode => {
            let dec_args = DecArgs::new(all_args)?;
            let privkey = rsa::PrivKey::new(&fs::read_to_string(&dec_args.priv_filename).or_else(|_| Err("Error reading privkey file"))?)?;
            let ciphertext = fs::read(&dec_args.ciphertext_filename).or_else(|_| Err("Error reading ciphertext file"))?;
            let plaintext = rsa::decode_text(&ciphertext, &privkey)?;
            fs::write(&dec_args.plaintext_filename, plaintext).or_else(|_| Err("Error writing to plaintext file"))?;
        },
        Type::Encode => {
            let enc_args = EncArgs::new(all_args)?;
            let pubkey = rsa::PubKey::new(&fs::read_to_string(&enc_args.pub_filename).or_else(|_| Err("Error reading pubkey file"))?)?;
            let plaintext = fs::read(&enc_args.plaintext_filename).or_else(|_| Err("Error reading plaintext file"))?;
            let ciphertext = rsa::encode_text(&plaintext, &pubkey)?;
            fs::write(&enc_args.ciphertext_filename, ciphertext).or_else(|_| Err("Error writing to ciphertext file"))?;
        }
    }

    Ok(())
}
