use std::env;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().compact().init();

    let addr = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let listener = TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Server listening on {}", &addr);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            tracing::info!("New client connection from {:?}", addr);
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
