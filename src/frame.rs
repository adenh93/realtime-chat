#[derive(Debug, Clone)]
pub enum Frame {
    Message(String),
    ServerMessage(String),
    Error(String),
}

impl Frame {
    pub fn frame_format(self) -> (u8, String, usize) {
        let (prefix, message) = match self {
            Frame::Message(msg) => (b'+', msg),
            Frame::ServerMessage(msg) => (b'$', msg),
            Frame::Error(msg) => (b'-', msg),
        };

        let length = message.len();

        (prefix, message, length)
    }

    pub fn message(self) -> String {
        match self {
            Frame::Message(msg) => msg,
            Frame::ServerMessage(msg) => msg,
            Frame::Error(msg) => msg,
        }
    }

    pub fn try_from_prefix(prefix: char, message: &str) -> Result<Self, std::io::Error> {
        let message = message.into();

        Ok(match prefix {
            '+' => Self::Message(message),
            '$' => Self::ServerMessage(message),
            '-' => Self::Error(message),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid message frame"),
            ))?,
        })
    }
}
