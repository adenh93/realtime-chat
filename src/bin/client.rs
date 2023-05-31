use clap::Parser;
use futures::{SinkExt, StreamExt};
use realtime_chat::{codec::MessageCodec, frame::Frame};
use tokio::{io::stdin, net::TcpStream};
use tokio_util::codec::{Framed, FramedRead, LinesCodec};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Address of server to connect to.
    #[arg(short, long, default_value_t = String::from("127.0.0.1:8080"))]
    address: String,

    /// Friendly nickname to display to other users.
    #[arg(short, long)]
    nickname: String,

    /// Password used to generate Tripcode. This will allow
    /// you to claim a unique username.
    #[arg(short, long)]
    password: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let socket = TcpStream::connect(args.address).await.unwrap();
    let mut lines = FramedRead::new(stdin(), LinesCodec::new());
    let mut messages = Framed::new(socket, MessageCodec {});

    let username_pair = format!("{},{}", args.nickname, args.password);
    messages.send(Frame::Message(username_pair)).await.unwrap();

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
