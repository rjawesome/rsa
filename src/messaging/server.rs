use std::error::Error;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpListener;

use crate::messaging::utils::{decode_message, u8_arr_to_len};
use crate::rsa;

pub fn run_server(port: u16) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    let (pubkey, privkey) = rsa::generate_keys()?;
    println!("Server Started");
    for stream in listener.incoming() {
        println!("Client Connected!");
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
            Err(_) => continue
        };
        let enc_key_len = u8_arr_to_len(enc_key_len);
        if enc_key_len == 0 {
            println!("Client Disconnected (key exchange)!");
            continue
        }

        // get AES key
        let mut enc_aes_key = vec![0; enc_key_len];
        match reader.read_exact(&mut enc_aes_key) {
            Ok(_) => (),
            Err(_) => continue
        };
        let aes_key = rsa::decode_text(&enc_aes_key, &privkey)?;

        // recieve messages
        loop {
            let mut len: [u8; 2] = [0; 2];
            match reader.read_exact(&mut len) {
                Ok(_) => (),
                Err(_) => break
            };
            let len = u8_arr_to_len(len);
            let mut iv: [u8; 16] = [0; 16];
            match reader.read_exact(&mut iv) {
                Ok(_) => (),
                Err(_) => break
            };
            let mut enc_message: Vec<u8> = vec![0; len];
            match reader.read_exact(&mut enc_message) {
                Ok(_) => (),
                Err(_) => break
            };
            let message = decode_message(&iv, &aes_key[0..16].try_into().unwrap(), &enc_message);
            print!("Client: {}", message);
        }
        println!("Client Disconnected!");
    }
    Ok(())
}