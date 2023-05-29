use super::{Peer, State, Username};
use crate::message::Message;
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};

pub type Lines = Framed<TcpStream, LinesCodec>;

#[derive(Debug)]
pub struct Connection {
    pub peer: Peer,
    pub lines: Lines,
    pub state: State,
}

impl Connection {
    pub async fn new(socket: TcpStream, addr: SocketAddr, state: State) -> Result<Self, String> {
        // TODO: Implement custom codec
        let mut lines = Framed::new(socket, LinesCodec::new());

        let username = Username::from_frame(&mut lines).await?;
        let peer = Peer::new(username, addr, state.clone()).await;
        let connection = Self { peer, lines, state };

        connection.on_connect().await;

        Ok(connection)
    }

    pub async fn process(&mut self) {
        loop {
            tokio::select! {
                Some(message) = self.peer.rx.recv() => {
                    self.lines.send(&message).await.unwrap();
                },
                result = self.lines.next() => match result {
                    Some(Ok(message)) => {
                        self.handle_incoming_message(message).await;
                    },
                    Some(Err(e)) => {
                        tracing::error!(
                            "An error occured while reading message from client at {}: {:?}", self.peer.addr, e
                        );
                    },
                    None => break
                }
            }
        }

        // TODO: Once async trait fns are in Stable Rust, move this
        // into Drop implementation.
        self.on_disconnect().await;
    }

    pub async fn handle_incoming_message(&mut self, message: String) {
        match Message::try_from(message) {
            Ok(Message::Cmd(cmd_type)) => {
                if let Err(err) = cmd_type.apply(self).await {
                    let _ = self.lines.send(format!("{err}")).await;
                }
            }
            Ok(Message::Raw(msg)) => {
                let mut state = self.state.lock().await;
                let message = format!("{}: {}", &self.peer.username.0, &msg);

                state.broadcast(self.peer.addr, &message).await;
            }
            Err(err) => {
                let _ = self.lines.send(format!("{err}")).await;
            }
        }
    }

    pub async fn on_connect(&self) {
        let mut state = self.state.lock().await;
        let message = format!("{} has joined the chat", self.peer.username.0);

        tracing::info!("{}", message);
        state.broadcast(self.peer.addr, &message).await;
    }

    pub async fn on_disconnect(&self) {
        let mut state = self.state.lock().await;
        let message = format!("{} has left the chat", &self.peer.username.0);

        state.peers.remove(&self.peer.username);
        tracing::info!("{}", message);
        state.broadcast(self.peer.addr, &message).await;
    }
}
