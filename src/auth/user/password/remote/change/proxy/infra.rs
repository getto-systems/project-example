use crate::auth::user::password::remote::change::infra::ChangePasswordFieldsExtract;

use crate::z_lib::remote::message::data::MessageError;

pub enum ChangePasswordProxyResponse {
    Success,
    InvalidPassword,
}

pub trait ChangePasswordProxyRequestDecoder {
    fn decode(self) -> Result<ChangePasswordFieldsExtract, MessageError>;
}
