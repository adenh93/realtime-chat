use crate::frame::Frame;

use super::{PeerConnection, State, Username};
use std::net::SocketAddr;
use tokio::sync::mpsc;

pub type Rx = mpsc::Receiver<Frame>;

const CHANNEL_BUFFER: usize = 64;

#[derive(Debug)]
pub struct Peer {
    pub username: Username,
    pub addr: SocketAddr,
    pub rx: Rx,
}

impl Peer {
    pub async fn new(username: Username, addr: SocketAddr, state: State) -> Self {
        let (tx, rx) = mpsc::channel(CHANNEL_BUFFER);

        let peer = Self {
            username: username.clone(),
            addr,
            rx,
        };

        let mut state = state.lock().await;
        let peer_connection = PeerConnection::new(addr, tx);

        state.peers.insert(username, peer_connection);

        peer
    }
}
