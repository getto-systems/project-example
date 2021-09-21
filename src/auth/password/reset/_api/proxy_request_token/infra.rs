use crate::auth::password::reset::_common::request_token::infra::RequestResetTokenFieldsExtract;

use crate::z_details::_api::message::data::MessageError;

pub enum RequestResetTokenProxyResponse {
    Success,
    InvalidRequest,
}

pub trait RequestResetTokenProxyRequestDecoder {
    fn decode(self) -> Result<RequestResetTokenFieldsExtract, MessageError>;
}
