use futures::{SinkExt, StreamExt};
use realtime_chat::{codec::MessageCodec, frame::Frame};
use std::env;
use tokio::{io::stdin, net::TcpStream};
use tokio_util::codec::{Framed, FramedRead, LinesCodec};

#[tokio::main]
async fn main() {
    let addr = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let socket = TcpStream::connect(addr).await.unwrap();
    let mut lines = FramedRead::new(stdin(), LinesCodec::new());
    let mut messages = Framed::new(socket, MessageCodec {});

    loop {
        tokio::select! {
            Some(Ok(input)) = lines.next() => {
                let frame = Frame::Message(input);
                let _ = messages.send(frame).await;
            },
            result = messages.next() => match result {
                Some(Ok(frame)) => {
                    let message = frame.message();
                    println!("{}", message);
                },
                Some(Err(e)) => {
                    eprintln!("An error occured: {:?}", e);
                },
                None => break
            },
        }
    }
}
