use std::error::Error;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

use rand::Rng;

use crate::rsa;

use super::utils::len_to_u8_arr;

pub fn run_client(host: &str, port: u16) -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect(format!("{}:{}", host, port))?;
    let mut reader = BufReader::new(&stream);
    let mut writer= BufWriter::new(&stream);

    // get pubkey
    let mut pubkey_vec = Vec::<u8>::new();
    let sz = reader.read_until(b'\n', &mut pubkey_vec)?;
    if sz == 0 {
        return Err("Server Disconnected")?;
    }
    let pubkey_str = String::from_utf8(pubkey_vec)?;
    let pubkey = rsa::PubKey::new(&pubkey_str[0..pubkey_str.len()-1])?;

    // generate AES key
    let mut rng = rand::thread_rng();
    let mut aes_key: [u8; 16] = [0; 16];
    for i in 0..16 {
        aes_key[i] = rng.gen_range(0..=255);
    }

    // encrypt AES key
    let enc_aes_key = rsa::encode_text(&aes_key, &pubkey)?;
    let enc_key_len = len_to_u8_arr(enc_aes_key.len());

    // transmit AES key
    writer.write(&enc_key_len)?;
    writer.write(&enc_aes_key)?;
    writer.flush()?;

    // print aes key
    for byte in aes_key {
        print!("{} ", byte);
    }
    println!("");

    Ok(())
}