use futures_util::StreamExt;
use tokio::net::TcpListener;
use std::{time::{SystemTime, UNIX_EPOCH}, fs, io::{Seek, Write}};

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            
            let listener = TcpListener::bind("127.0.0.1:8080")
                .await
                .expect("Listening to TCP failed.");

            let mut y = fs::File::create("/Users/logpoint/rust_cn/index").unwrap();
            let mut z = fs::File::create("/Users/logpoint/rust_cn/log").unwrap();

            while let Ok((stream, peer)) = listener.accept().await {
                match tokio_tungstenite::accept_async(stream).await {
                    Err(e) => println!("Error during the websocket handshake occurred: {}", e),
                    Ok(ws_stream) => {
                        println!("New Connection : {}", peer);
                        let (_sender, mut receiver) = ws_stream.split();
                        while let Some(msg) = receiver.next().await {
                            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                            let start_ptr = z.stream_position().unwrap();
                            z.write(format!("{}\n", msg.unwrap()).as_bytes()).unwrap();
                            let end_ptr = z.stream_position().unwrap();
                            y.write(format!("{timestamp},{start_ptr},{end_ptr}\n").as_bytes()).unwrap();
                        }
                    }
                }
            }
        })
}