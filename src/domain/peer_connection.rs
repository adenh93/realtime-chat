use std::net::SocketAddr;
use tokio::sync::mpsc;

pub type Tx = mpsc::Sender<String>;

#[derive(Debug)]
pub struct PeerConnection {
    pub addr: SocketAddr,
    pub tx: Tx,
}

impl PeerConnection {
    pub fn new(addr: SocketAddr, tx: Tx) -> Self {
        Self { addr, tx }
    }
}
