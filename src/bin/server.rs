use std::env;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let addr = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let listener = TcpListener::bind(&addr).await.unwrap();

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            process_connection(&mut socket).await;
        });
    }
}

async fn process_connection(socket: &mut TcpStream) {
    let (mut rd, mut wr) = socket.split();

    loop {
        tokio::io::copy(&mut rd, &mut wr).await.unwrap();
    }
}
