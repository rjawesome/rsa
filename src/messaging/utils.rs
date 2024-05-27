use std::error::Error;

use tokio::{io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter}, net::tcp::{OwnedReadHalf, OwnedWriteHalf}};

use rand::Rng;
use libaes::Cipher;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f";
        let message = "Hello World!";
        let encoded = generate_message(message, key);

        let encoded_len = u8_arr_to_len(encoded[0..2].try_into().unwrap());
        assert_eq!(encoded_len, encoded.len() - 18);

        let decoded = decode_message(&encoded[2..18], key, &encoded[18..]);
        assert_eq!(decoded, message);
    }
}


pub fn len_to_u8_arr(num: usize) -> [u8; 2] {
    let len_1 = (num & 255).try_into().unwrap();
    let len_2 = ((num >> 8) & 255).try_into().unwrap();
    return [len_1, len_2];
}

pub fn u8_arr_to_len(arr: [u8; 2]) -> usize {
    let len_1: usize = arr[0].into();
    let len_2: usize = arr[1].into();
    return (len_2 << 8) + len_1;
}

pub fn generate_message(msg: &str, key: &[u8; 16]) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut iv: [u8; 16] = [0; 16];
    for i in 0..16 {
        iv[i] = rng.gen_range(0..=255);
    }

    let cipher = Cipher::new_128(key);
    let mut encrypted = cipher.cbc_encrypt(&iv, &msg.as_bytes());
    let len = len_to_u8_arr(encrypted.len());

    let mut data = vec![len[0], len[1]];
    data.extend_from_slice(&iv);
    data.append(&mut encrypted);

    data
}

pub fn decode_message(iv: &[u8], key: &[u8; 16], msg: &[u8]) -> String {
    let cipher = Cipher::new_128(key);
    String::from_utf8(cipher.cbc_decrypt(iv, msg)).unwrap()
}

pub async fn recieve_messages(reader: &mut BufReader<OwnedReadHalf>, aes_key: &[u8; 16]) {
    loop {
        let mut len: [u8; 2] = [0; 2];
        match reader.read_exact(&mut len).await {
            Ok(_) => (),
            Err(_) => break
        };
        let len = u8_arr_to_len(len);
        let mut iv: [u8; 16] = [0; 16];
        match reader.read_exact(&mut iv).await {
            Ok(_) => (),
            Err(_) => break
        };
        let mut enc_message: Vec<u8> = vec![0; len];
        match reader.read_exact(&mut enc_message).await {
            Ok(_) => (),
            Err(_) => break
        };
        let message = decode_message(&iv, aes_key, &enc_message);
        println!("> {}", message.trim());
    }
}

#[tokio::main]
pub async fn send_messages(writer: &mut BufWriter<OwnedWriteHalf>, aes_key: &[u8; 16]) -> Result<(), Box<dyn Error>> {
    loop {
        let mut string = String::new();
        std::io::stdin().read_line(&mut string)?;
        if string.trim() == "quit" {
            break
        }
        match writer.write(&generate_message(&string, &aes_key)).await {
            Ok(_) => (),
            Err(_) => break
        }
        match writer.flush().await {
            Ok(_) => (),
            Err(_) => break
        };
    }
    Ok(())
}