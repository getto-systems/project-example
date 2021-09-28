use crate::z_lib::remote::message::data::MessageError;

pub struct RequestResetTokenFieldsExtract {
    pub login_id: String,
}

pub enum RequestResetTokenProxyResponse {
    Success,
    InvalidRequest,
}

pub trait RequestResetTokenProxyRequestDecoder {
    fn decode(self) -> Result<RequestResetTokenFieldsExtract, MessageError>;
}
