use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::{select, task};

use crate::messaging::utils::{check_alive, read_string, read_vec, write_arr, write_string, ClientInfo};

#[tokio::main]
pub async fn run_new_server(port: u16) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("Server Started");
    let clients = Arc::new(Mutex::new(HashMap::<String, ClientInfo>::new()));
    loop {
        let mut stream = listener.accept().await?.0;
        let clients = Arc::clone(&clients);
        let _: JoinHandle<Result<(), &'static str>> = task::spawn(async move {
            let src_username = read_string(&mut stream).await?;
            let dst_username = read_string(&mut stream).await?;
            let mut clients = clients.lock().await;

            if clients.contains_key(&src_username) {
                let old_stream = &mut clients.get_mut(&src_username).unwrap().stream;
                if check_alive(old_stream).await {
                    write_string(&mut stream, "taken").await?;
                    return Ok(())
                } else {
                    clients.remove(&src_username);
                }
            }

            if !clients.contains_key(&dst_username) || clients.get(&dst_username).unwrap().destination != src_username {
                write_string(&mut stream, "wait").await?;
                clients.insert(src_username.clone(), ClientInfo { stream, destination: dst_username.clone() });
                return Ok(())
            }

            let mut other_stream = clients.remove(&dst_username).unwrap().stream;
            drop(clients); // lock not needed

            write_string(&mut stream, "key").await?;
            let pubkey = read_string(&mut stream).await?;
            write_string(&mut other_stream, &pubkey).await?;
            let aes_key = read_vec(&mut other_stream).await?;
            write_arr(&mut stream, &aes_key).await?;

            loop {
                select! {
                    vec_opt = read_vec(&mut stream) => {
                        if let Ok(vec) = vec_opt {
                            if vec.len() > 0 {
                                write_arr(&mut other_stream, &vec).await?;
                                continue
                            }
                        }
                        break
                    },
                    vec_opt = read_vec(&mut other_stream) => {
                        if let Ok(vec) = vec_opt {
                            if vec.len() > 0 {
                                write_arr(&mut stream, &vec).await?;
                                continue
                            }
                        }
                        break
                    }
                }
            }

            Ok(())
        });
    }
}