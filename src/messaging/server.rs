use std::error::Error;
use std::io::{BufReader, BufWriter, ErrorKind, Read, Write};
use std::net::TcpListener;

use crate::messaging::utils::u8_arr_to_len;
use crate::rsa;

pub fn run_server(port: u16) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    let (pubkey, privkey) = rsa::generate_keys()?;
    println!("Server Started");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let mut reader = BufReader::new(&stream);
        let mut writer= BufWriter::new(&stream);

        // write pubkey
        writer.write(pubkey.to_string().as_bytes())?;
        writer.write(&[b'\n'])?;
        writer.flush()?;

        // get AES key len
        let mut enc_key_len: [u8; 2] = [0; 2];
        match reader.read_exact(&mut enc_key_len) {
            Ok(_) => (),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => continue,
            Err(e) => Err(e)?
        }
        let enc_key_len = u8_arr_to_len(enc_key_len);
        if enc_key_len == 0 {
            continue
        }

        // get AES key
        let mut enc_aes_key = vec![0; enc_key_len];
        match reader.read_exact(&mut enc_aes_key) {
            Ok(_) => (),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => continue,
            Err(e) => Err(e)?
        }
        let aes_key = rsa::decode_text(&enc_aes_key, &privkey)?;

        // print aes key
        for byte in aes_key {
            print!("{} ", byte);
        }
        println!("");
    }
    Ok(())
}