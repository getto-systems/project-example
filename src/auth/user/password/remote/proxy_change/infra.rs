use crate::z_details::_api::message::data::MessageError;

pub struct ChangePasswordFieldsExtract {
    pub current_password: String,
    pub new_password: String,
}

pub enum ChangePasswordProxyResponse {
    Success,
    InvalidPassword,
}

pub trait ChangePasswordProxyRequestDecoder {
    fn decode(self) -> Result<ChangePasswordFieldsExtract, MessageError>;
}
