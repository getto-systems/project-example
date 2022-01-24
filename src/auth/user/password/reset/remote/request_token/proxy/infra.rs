use crate::auth::user::password::reset::remote::request_token::infra::RequestResetTokenFieldsExtract;

use crate::z_lib::remote::message::data::MessageError;

pub enum RequestResetTokenProxyResponse {
    Success,
    InvalidRequest,
}

pub trait RequestResetTokenProxyRequestDecoder {
    fn decode(self) -> Result<RequestResetTokenFieldsExtract, MessageError>;
}
