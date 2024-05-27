use std::error::Error;
use std::thread;

use rand::Rng;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::{join, select, task};
use tokio::sync::mpsc::channel;

use crate::messaging::utils::{recieve_messages, send_messages};
use crate::rsa;

use super::utils::len_to_u8_arr;

#[tokio::main]
pub async fn run_client(host: &str, port: u16) -> Result<(), Box<dyn Error>> {
    let main_stream = TcpStream::connect(format!("{}:{}", host, port)).await?;
    let (read_stream, write_stream) = main_stream.into_split();
    let mut reader = BufReader::new(read_stream);
    let mut writer= BufWriter::new(write_stream);

    // get pubkey
    let mut pubkey_vec = Vec::<u8>::new();
    let sz = reader.read_until(b'\n', &mut pubkey_vec).await?;
    if sz == 0 {
        return Err("Server Disconnected")?;
    }
    let pubkey_str = String::from_utf8(pubkey_vec)?;
    let pubkey = rsa::PubKey::new(&pubkey_str[0..pubkey_str.len()-1])?;

    // generate AES key
    let mut rng = rand::thread_rng();
    let mut aes_key: [u8; 16] = [0; 16];
    let mut aes_key_2: [u8; 16] = [0; 16];
    for i in 0..16 {
        aes_key[i] = rng.gen_range(0..=255);
        aes_key_2[i] = aes_key[i];
    }

    // encrypt AES key
    let enc_aes_key = rsa::encode_text(&aes_key, &pubkey)?;
    let enc_key_len = len_to_u8_arr(enc_aes_key.len());

    // transmit AES key
    writer.write(&enc_key_len).await?;
    writer.write(&enc_aes_key).await?;
    writer.flush().await?;

    println!("Connected to Server ('quit' to exit)!");

    // messaging
    let aes_key_ref = &aes_key[0..16];
    let reciever = recieve_messages(&mut reader, aes_key_ref.try_into()?);
    // let (sender_tx, mut sender_rx) = channel::<i32>(1);
    let th = task::spawn_blocking(move || {
        send_messages(&mut writer, &aes_key_2[0..16].try_into().unwrap()).unwrap();
    });

    select! {
        () = reciever => {
            drop(reader);
        },
        _ = th => {
            drop(reader);
        }
    }

    println!("Server disconnected!");

    Ok(())
}