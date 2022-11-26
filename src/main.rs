mod cn;
mod repo;
mod models;

use futures::SinkExt;
use json::object;
use cn::_instance;
use repo::mongo_repo::Mongo;
use futures_util::StreamExt;
use tokio::{net::TcpListener, io::AsyncReadExt};
use tokio_tungstenite::tungstenite::Message;
use std::{time::Instant, collections::HashMap};

#[allow(unused_must_use)]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = Mongo::init().await;
    let (_processor, _parser, _normalizer) = _instance::init(db).await;
    // let _raw_logs = r#""#;
    let listener = TcpListener::bind("127.0.0.1:5500")
                .await
                .expect("Listening to TCP failed.");
    let start_time = Instant::now();
    let mut str_log = object! {};
    let mut normalized_log = object! {};
    let mut typemapper = HashMap::new();
    let (ws_stream, _response) = tokio_tungstenite::connect_async("ws://127.0.0.1:8080").await.expect("Error during the websocket handshake occurred");
    let (mut _sender, _) = ws_stream.split();
    let mut buf = String::new();
    while let Ok((mut stream, peer)) = listener.accept().await {
        println!("Connected to peer: {}", peer);
        stream.read_to_string(&mut buf).await?;
        let mut raw_logs = buf.lines();
        while let Some(raw_log) = raw_logs
        .next()
        {
            if let Some(id) = _processor.pre_process(&raw_log) {
                _parser.parse(&id, &raw_log, &mut str_log);
                _processor.post_process(&id, &raw_log, &mut str_log);
                _normalizer._normalize(&str_log, &id, &mut normalized_log, &mut typemapper);
    
                _sender.send(Message::Binary(normalized_log.to_string().into_bytes())).await;
                _sender.close();
                
                str_log.clear();
                normalized_log.clear();
                typemapper.clear();
            }
        }
        buf.clear();
    }
    
    println!("{:#?}", start_time.elapsed());
    Ok(())
}
