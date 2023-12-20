use prost::DecodeError;

#[derive(Debug)]
pub enum MessageError {
    ProtobufDecodeError(DecodeError),
    Invalid(String),
}

impl std::fmt::Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::ProtobufDecodeError(err) => write!(f, "protobuf decode error: {}", err),
            Self::Invalid(err) => write!(f, "invalid message: {}", err),
        }
    }
}

impl From<DecodeError> for MessageError {
    fn from(value: DecodeError) -> Self {
        Self::ProtobufDecodeError(value)
    }
}
