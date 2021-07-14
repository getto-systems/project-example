use std::collections::VecDeque;

use base64::{decode_config, encode_config, STANDARD};
use bytes::Buf;
use prost::{DecodeError, Message as ProstMessage};

use super::data::MessageError;

pub fn encode_protobuf_base64(message: impl ProstMessage) -> Result<String, MessageError> {
    let mut bytes: Vec<u8> = vec![];
    message
        .encode(&mut bytes)
        .map_err(|err| MessageError::Invalid(format!("failed to encode protobuf; {}", err)))?;
    Ok(encode_config(bytes, STANDARD))
}

pub fn decode_base64(content: String) -> Result<impl Buf, MessageError> {
    let buf = decode_config(content, STANDARD)
        .map_err(|err| MessageError::Invalid(format!("failed to decode base64; {}", err)))?;
    let buf: VecDeque<u8> = buf.into();
    Ok(buf)
}
pub fn invalid_protobuf(err: DecodeError) -> MessageError {
    MessageError::Invalid(format!("failed to decode protobuf; {}", err))
}
