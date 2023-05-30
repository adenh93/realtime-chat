use super::{PeerConnection, Username};
use crate::frame::Frame;
use futures::future::join_all;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

pub type State = Arc<Mutex<Shared>>;

#[derive(Debug)]
pub struct Shared {
    pub peers: HashMap<Username, PeerConnection>,
}

impl Shared {
    pub fn new() -> Self {
        Shared {
            peers: HashMap::new(),
        }
    }

    pub async fn broadcast(&mut self, sender: SocketAddr, frame: Frame) {
        // TODO: Maybe allow the caller to specify if the sender should also receive the message?
        let filtered_peers = self.peers.iter().filter(|peer| peer.1.addr != sender);

        // TODO: Remove this clone call when passing the frame to the peer's sender
        let futs = filtered_peers.map(|peer| peer.1.tx.send(frame.clone()));
        join_all(futs).await;
    }
}
