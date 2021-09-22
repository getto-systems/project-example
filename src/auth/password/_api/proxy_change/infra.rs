use crate::auth::password::_common::change::infra::ChangePasswordFieldsExtract;

use crate::z_details::_api::message::data::MessageError;

pub enum ChangePasswordProxyResponse {
    Success,
    InvalidPassword,
}

pub trait ChangePasswordProxyRequestDecoder {
    fn decode(self) -> Result<ChangePasswordFieldsExtract, MessageError>;
}
