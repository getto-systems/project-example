use prost::Message;

use crate::auth::user::password::_api::y_protobuf::api::AuthenticatePasswordPb;

use crate::z_details::_api::message::helper::{decode_base64, invalid_protobuf};

use crate::auth::user::password::remote::proxy_authenticate::infra::{
    AuthenticatePasswordFieldsExtract, AuthenticatePasswordProxyRequestDecoder,
};

use crate::z_details::_api::message::data::MessageError;

pub struct RequestDecoder {
    body: String,
}

impl RequestDecoder {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl AuthenticatePasswordProxyRequestDecoder for RequestDecoder {
    fn decode(self) -> Result<AuthenticatePasswordFieldsExtract, MessageError> {
        let message =
            AuthenticatePasswordPb::decode(decode_base64(self.body)?).map_err(invalid_protobuf)?;

        Ok(AuthenticatePasswordFieldsExtract {
            login_id: message.login_id,
            password: message.password,
        })
    }
}
