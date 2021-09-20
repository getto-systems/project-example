use prost::Message;

use crate::auth::password::_api::y_protobuf::api::AuthenticatePasswordPb;

use crate::z_details::_api::message::helper::{decode_base64, invalid_protobuf};

use super::AuthenticatePasswordRequestDecoder;
use crate::auth::password::_common::authenticate::infra::AuthenticatePasswordFieldsExtract;

use crate::z_details::_api::message::data::MessageError;

pub struct AuthenticateRequestDecoder {
    body: String,
}

impl AuthenticateRequestDecoder {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl AuthenticatePasswordRequestDecoder for AuthenticateRequestDecoder {
    fn decode(self) -> Result<AuthenticatePasswordFieldsExtract, MessageError> {
        let message =
            AuthenticatePasswordPb::decode(decode_base64(self.body)?).map_err(invalid_protobuf)?;

        Ok(AuthenticatePasswordFieldsExtract {
            login_id: message.login_id,
            password: message.password,
        })
    }
}
