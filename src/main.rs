use realtime_chat::domain::{Connection, Shared};
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().compact().init();

    let listener = get_listener().await;
    let state = Arc::new(Mutex::new(Shared::new()));

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        let state = state.clone();

        tokio::spawn(async move {
            tracing::info!("New client connection from {:?}", addr);

            match Connection::new(socket, addr, state).await {
                Ok(mut conn) => conn.process().await,
                Err(e) => {
                    tracing::error!("{}", e);
                    return;
                }
            };
        });
    }
}

async fn get_listener() -> TcpListener {
    let addr = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let listener = TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Server listening on {}", &addr);

    listener
}
