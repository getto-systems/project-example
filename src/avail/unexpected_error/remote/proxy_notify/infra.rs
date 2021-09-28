use crate::z_lib::remote::message::data::MessageError;

pub struct NotifyUnexpectedErrorFieldsExtract {
    pub err: String,
}

pub trait NotifyUnexpectedErrorProxyRequestDecoder {
    fn decode(self) -> Result<NotifyUnexpectedErrorFieldsExtract, MessageError>;
}
