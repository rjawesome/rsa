use std::error::Error;
use std::thread;

use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpListener;
use tokio::{select, task};

use crate::messaging::utils::{recieve_messages, send_messages, u8_arr_to_len};
use crate::rsa;

#[tokio::main]
pub async fn run_server(port: u16) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    let (pubkey, privkey) = rsa::generate_keys()?;
    println!("Server Started");
    loop {
        let main_stream = listener.accept().await?.0;
        let (read_stream, write_stream) = main_stream.into_split();
        println!("Client Connected!");
        let mut reader = BufReader::new(read_stream);
        let mut writer= BufWriter::new(write_stream);

        // write pubkey
        writer.write(pubkey.to_string().as_bytes()).await?;
        writer.write(&[b'\n']).await?;
        writer.flush().await?;

        // get AES key len
        let mut enc_key_len: [u8; 2] = [0; 2];
        match reader.read_exact(&mut enc_key_len).await {
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
        match reader.read_exact(&mut enc_aes_key).await {
            Ok(_) => (),
            Err(_) => continue
        };
        let aes_key = rsa::decode_text(&enc_aes_key, &privkey)?;
        let aes_key_2 = aes_key.clone();

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
        
        println!("Client Disconnected!");
    }
}