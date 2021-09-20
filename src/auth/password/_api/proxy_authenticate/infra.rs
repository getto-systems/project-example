use crate::{
    auth::password::_common::authenticate::infra::AuthenticatePasswordFieldsExtract,
    z_details::_api::message::data::MessageError,
};

pub trait AuthenticatePasswordProxyRequestDecoder {
    fn decode(self) -> Result<AuthenticatePasswordFieldsExtract, MessageError>;
}
