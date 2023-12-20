use std::collections::VecDeque;

use base64::{engine::general_purpose::STANDARD, Engine};
use prost::Message;

use super::data::MessageError;

pub fn encode_protobuf_base64(message: impl Message) -> Result<String, MessageError> {
    let mut bytes: Vec<u8> = vec![];
    message
        .encode(&mut bytes)
        .map_err(|err| MessageError::Invalid(format!("failed to encode protobuf; {}", err)))?;
    Ok(STANDARD.encode(bytes))
}

pub fn decode_base64(content: String) -> Result<VecDeque<u8>, MessageError> {
    let buf = STANDARD
        .decode(content)
        .map_err(|err| MessageError::Invalid(format!("failed to decode base64; {}", err)))?;
    Ok(buf.into())
}
