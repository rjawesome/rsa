use std::error::Error;

use rand::Rng;
use tokio::io::{BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::{select, task};

use crate::messaging::utils::{read_string, read_vec, recieve_messages, send_messages, write_arr, write_string, AesKey};
use crate::rsa::{self, PubKey};

#[tokio::main]
pub async fn run_new_client(host: &str, port: u16, src_username: &str, dst_username: &str) -> Result<(), Box<dyn Error>> {
    let (mut pubkey, privkey) = rsa::generate_keys()?;
    println!("Client Started!");
    let mut main_stream = TcpStream::connect(format!("{}:{}", host, port)).await?;
    let mut aes_key_1: AesKey = AesKey::Array([0; 16]);
    let mut aes_key_2: AesKey = AesKey::Array([0; 16]);

    write_string(&mut main_stream, src_username).await?;
    write_string(&mut main_stream, dst_username).await?;

    let result = read_string(&mut main_stream).await?;
    match result.as_str() {
        "taken" => {
            println!("Username taken!");
            return Ok(());
        },
        "wait" => {
            println!("Waiting for other person to join!");
            let pubkey_str = loop {
                let input = read_string(&mut main_stream).await?;
                if &input == "alive" {
                    write_string(&mut main_stream, "alive").await?;
                    continue
                }
                break input
            };
            pubkey = PubKey::new(&pubkey_str)?;
    
            // generate AES key
            let mut rng = rand::thread_rng();
            for i in 0..16 {
                aes_key_1[i] = rng.gen_range(0..=255);
                aes_key_2[i] = aes_key_1[i];
            }
            // encrypt AES key
            let enc_aes_key = rsa::encode_text(match &aes_key_1 { AesKey::Array(x) => x, AesKey::Vector(x) => x }, &pubkey)?;
            write_arr(&mut main_stream, &enc_aes_key).await?;
        },
        "key" => {
            write_string(&mut main_stream, &pubkey.to_string()).await?;
            let enc_aes_key = read_vec(&mut main_stream).await?;
            let aes_key = rsa::decode_text(&enc_aes_key, &privkey)?;
            aes_key_1 = AesKey::Vector(aes_key.clone());
            aes_key_2 = AesKey::Vector(aes_key);
        }
        _ => {
            println!("Server returned unexpected contents!");
            return Ok(());
        }
    }

    println!("Connected to other client ('quit' to exit)!");
    let hash = md5::compute(pubkey.to_string().as_bytes());
    println!("Verification Code (should match between you and other client): {:x}", hash);

    // messaging
    let (read_stream, write_stream) = main_stream.into_split();
    let mut reader = BufReader::new(read_stream);
    let mut writer= BufWriter::new(write_stream);

    let aes_key_ref: &[u8] = match &aes_key_1 { AesKey::Array(x) => x, AesKey::Vector(x) => x };
    let reciever = recieve_messages(&mut reader, aes_key_ref.try_into()?);

    let sender = task::spawn_blocking(move || {
        let aes_key_ref: &[u8] = match &aes_key_2 { AesKey::Array(x) => x, AesKey::Vector(x) => x };
        send_messages(&mut writer, aes_key_ref.try_into().unwrap()).unwrap();
    });

    select! {
        () = reciever => {
            drop(reader);
        },
        _ = sender => {
            drop(reader);
        }
    }

    println!("Server or other client disconnected!");

    Ok(())
}