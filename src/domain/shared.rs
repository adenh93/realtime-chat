use super::{PeerConnection, Username};
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

    pub async fn broadcast(&mut self, sender: SocketAddr, message: &str) {
        // TODO: Maybe allow the caller to specify if the sender should also receive the message?
        let filtered_peers = self.peers.iter().filter(|peer| peer.1.addr != sender);
        let futs = filtered_peers.map(|peer| peer.1.tx.send(message.into()));
        join_all(futs).await;
    }
}
