use crate::frame::Frame;
use bytes::{Buf, BufMut, BytesMut};
use std::io::{Error, ErrorKind};
use std::mem::size_of;
use tokio_util::codec::{Decoder, Encoder};

/// Custom codec for encoding/decoding custom message
/// frames.
#[derive(Debug)]
pub struct MessageCodec {}

// Reserved bytes for the prefix character
const PREFIX_BYTE_LEN: usize = 1;

// Reserved bytes for the length marker
const LENGTH_BYTE_LEN: usize = size_of::<u32>();

// Combined reserved bytes
const RESERVED_BYTES: usize = PREFIX_BYTE_LEN + LENGTH_BYTE_LEN;

// Maximum message length is 512 characters, regardless of
// type of frame
const MAX_LENGTH: usize = size_of::<char>() * 512;

impl Encoder<Frame> for MessageCodec {
    type Error = Error;

    fn encode(&mut self, item: Frame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // Extract the message and prefix for each type of frame.
        let (prefix, message, length) = item.frame_format();

        // Assure that the length of the message does not exceed MAX_LENGTH.
        if length > MAX_LENGTH {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Message is too large (length: {}).", length),
            ));
        }

        // Convert the length of the message into a byte array.
        let length_slice = u32::to_le_bytes(length as u32);

        // Reserve space in the buffer.
        dst.reserve(RESERVED_BYTES + length);

        // Write the prefix, length and message bytes to the buffer.
        dst.put_u8(prefix);
        dst.extend_from_slice(&length_slice);
        dst.extend_from_slice(message.as_bytes());

        Ok(())
    }
}

impl Decoder for MessageCodec {
    type Item = Frame;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < RESERVED_BYTES {
            // Not enough data to read prefix and length markers.
            return Ok(None);
        }

        // We can already guarantee that this unwrap will succeed
        let prefix_byte = src.first().unwrap();
        let mut length_bytes = [0u8; LENGTH_BYTE_LEN];

        // Copy the length marker bytes from the src bytes into our slice
        length_bytes.copy_from_slice(&src[PREFIX_BYTE_LEN..RESERVED_BYTES]);

        // Parse the prefix character and message length
        let prefix = *prefix_byte as char;
        let length = u32::from_le_bytes(length_bytes) as usize;

        // Assure that the length of the frame that we're decoding doesn't
        // exceed the maximum allowed length
        if length > MAX_LENGTH {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Message is too large (length: {})", length),
            ));
        }

        // If the length of the frame is less than the marked length (plus the
        // size of the reserved bytes), then we still haven't decoded a complete
        // frame.
        if src.len() < RESERVED_BYTES + length {
            src.reserve(RESERVED_BYTES + length - src.len());
            return Ok(None);
        }

        // Populate a new vector of bytes with the slice of bytes containing the
        // message, and advance the internal buffer.
        let message_data = src[RESERVED_BYTES..RESERVED_BYTES + length].to_vec();
        src.advance(RESERVED_BYTES + length);

        let message = match String::from_utf8(message_data) {
            Ok(msg) => msg,
            Err(err) => Err(Error::new(ErrorKind::InvalidData, err.utf8_error()))?,
        };

        // Attempt to locate a corresponding Frame variant, and construct it.
        let frame = Frame::try_from_prefix(prefix, &message)?;

        Ok(Some(frame))
    }
}
