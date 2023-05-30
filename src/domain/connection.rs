use super::{Message, Peer, State, Username};
use crate::{codec::MessageCodec, frame::Frame};
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub type Messages = Framed<TcpStream, MessageCodec>;

#[derive(Debug)]
pub struct Connection {
    pub peer: Peer,
    pub messages: Messages,
    pub state: State,
}

impl Connection {
    pub async fn new(socket: TcpStream, addr: SocketAddr, state: State) -> Result<Self, String> {
        // TODO: Implement custom codec
        let mut messages = Framed::new(socket, MessageCodec {});

        let username = Username::from_frame(&mut messages).await?;
        let peer = Peer::new(username, addr, state.clone()).await;

        let connection = Self {
            peer,
            messages,
            state,
        };

        connection.on_connect().await;

        Ok(connection)
    }

    pub async fn process(&mut self) {
        loop {
            tokio::select! {
                Some(message) = self.peer.rx.recv() => {
                    self.messages.send(message).await.unwrap();
                },
                result = self.messages.next() => match result {
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

    pub async fn handle_incoming_message(&mut self, message: Frame) {
        match Message::try_from(message) {
            Ok(Message::Cmd(cmd_type)) => {
                if let Err(err) = cmd_type.apply(self).await {
                    let frame = Frame::Error(err.to_string());
                    let _ = self.messages.send(frame).await;
                }
            }
            Ok(Message::Raw(msg)) => {
                let mut state = self.state.lock().await;
                let message = format!("{}: {}", &self.peer.username.0, &msg);
                let frame = Frame::Message(message);

                state.broadcast(self.peer.addr, frame).await;
            }
            Err(err) => {
                let frame = Frame::Error(err.to_string());
                let _ = self.messages.send(frame).await;
            }
        }
    }

    pub async fn on_connect(&self) {
        let mut state = self.state.lock().await;
        let message = format!("{} has joined the chat", self.peer.username.0);
        let frame = Frame::ServerMessage(message);

        state.broadcast(self.peer.addr, frame).await;
    }

    pub async fn on_disconnect(&self) {
        let mut state = self.state.lock().await;
        let message = format!("{} has left the chat", &self.peer.username.0);
        let frame = Frame::ServerMessage(message);

        state.peers.remove(&self.peer.username);
        state.broadcast(self.peer.addr, frame).await;
    }
}
