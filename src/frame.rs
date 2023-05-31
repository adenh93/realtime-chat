#[derive(Debug, PartialEq, Clone)]
pub enum Frame {
    Message(String),
    ServerMessage(String),
    PrivateMessage(String),
    Error(String),
}

impl Frame {
    pub fn frame_format(self) -> (u8, String, usize) {
        let (prefix, message) = match self {
            Frame::Message(msg) => (b'+', msg),
            Frame::ServerMessage(msg) => (b'$', msg),
            Frame::PrivateMessage(msg) => (b'&', msg),
            Frame::Error(msg) => (b'-', msg),
        };

        let length = message.len();

        (prefix, message, length)
    }

    pub fn message(self) -> String {
        match self {
            Frame::Message(msg) => msg,
            Frame::ServerMessage(msg) => msg,
            Frame::PrivateMessage(msg) => msg,
            Frame::Error(msg) => msg,
        }
    }

    pub fn try_from_prefix(prefix: char, message: &str) -> Result<Self, std::io::Error> {
        let message = message.into();

        Ok(match prefix {
            '+' => Self::Message(message),
            '$' => Self::ServerMessage(message),
            '&' => Self::PrivateMessage(message),
            '-' => Self::Error(message),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid message frame"),
            ))?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::{faker::lorem::en::Word, Fake};

    #[test]
    fn frame_format_returns_message_format() {
        let message = Word().fake::<String>();
        let length = message.len();

        let frame = Frame::Message(message.clone());
        let format = frame.frame_format();

        assert_eq!(format, (b'+', message, length));
    }

    #[test]
    fn frame_format_returns_server_message_format() {
        let message = Word().fake::<String>();
        let length = message.len();

        let frame = Frame::ServerMessage(message.clone());
        let format = frame.frame_format();

        assert_eq!(format, (b'$', message, length));
    }

    #[test]
    fn frame_format_returns_private_message_format() {
        let message = Word().fake::<String>();
        let length = message.len();

        let frame = Frame::PrivateMessage(message.clone());
        let format = frame.frame_format();

        assert_eq!(format, (b'&', message, length));
    }

    #[test]
    fn frame_format_returns_error_format() {
        let message = Word().fake::<String>();
        let length = message.len();

        let frame = Frame::Error(message.clone());
        let format = frame.frame_format();

        assert_eq!(format, (b'-', message, length));
    }

    #[test]
    fn message_extracts_frame_message() {
        let message = Word().fake::<String>();

        let message_frame = Frame::Message(message.clone());
        let server_message_frame = Frame::ServerMessage(message.clone());
        let private_message_frame = Frame::PrivateMessage(message.clone());
        let error_frame = Frame::Error(message.clone());

        assert_eq!(message_frame.message(), message);
        assert_eq!(server_message_frame.message(), message);
        assert_eq!(private_message_frame.message(), message);
        assert_eq!(error_frame.message(), message);
    }

    #[test]
    fn try_from_prefix_retrieves_message() {
        let message = Word().fake::<String>();
        let frame = Frame::try_from_prefix('+', &message).unwrap();

        assert_eq!(frame, Frame::Message(message));
    }

    #[test]
    fn try_from_prefix_retrieves_server_message() {
        let message = Word().fake::<String>();
        let frame = Frame::try_from_prefix('$', &message).unwrap();

        assert_eq!(frame, Frame::ServerMessage(message));
    }

    #[test]
    fn try_from_prefix_retrieves_private_message() {
        let message = Word().fake::<String>();
        let frame = Frame::try_from_prefix('&', &message).unwrap();

        assert_eq!(frame, Frame::PrivateMessage(message));
    }

    #[test]
    fn try_from_prefix_retrieves_error() {
        let message = Word().fake::<String>();
        let frame = Frame::try_from_prefix('-', &message).unwrap();

        assert_eq!(frame, Frame::Error(message));
    }

    #[test]
    fn try_from_prefix_returns_error_if_unknown_prefix() {
        let message = Word().fake::<String>();
        let frame = Frame::try_from_prefix('\0', &message);

        assert!(frame.is_err());
    }
}
