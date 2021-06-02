use base64::{decode_config, encode_config, STANDARD};
use protobuf::Message;

use super::data::MessageError;

pub fn encode_protobuf_base64(message: impl Message) -> Result<String, MessageError> {
    let bytes = message
        .write_to_bytes()
        .map_err(|err| MessageError::Invalid(format!("{}", err)))?;
    Ok(encode_config(bytes, STANDARD))
}
pub fn decode_protobuf_base64<M: Message>(message: String) -> Result<M, MessageError> {
    let bytes = decode_config(message, STANDARD)
        .map_err(|err| MessageError::Invalid(format!("{}", err)))?;
    Message::parse_from_bytes(&bytes).map_err(|err| MessageError::Invalid(format!("{}", err)))
}
