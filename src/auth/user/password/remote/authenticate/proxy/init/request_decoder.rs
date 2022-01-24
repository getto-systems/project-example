use prost::Message;

use crate::auth::user::password::remote::y_protobuf::api::AuthenticatePasswordApiRequestPb;

use crate::z_lib::remote::message::helper::{decode_base64, invalid_protobuf};

use crate::auth::user::password::remote::authenticate::{
    infra::AuthenticatePasswordFieldsExtract, proxy::infra::AuthenticatePasswordProxyRequestDecoder,
};

use crate::z_lib::remote::message::data::MessageError;

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
        let message = AuthenticatePasswordApiRequestPb::decode(decode_base64(self.body)?)
            .map_err(invalid_protobuf)?;

        Ok(AuthenticatePasswordFieldsExtract {
            login_id: message.login_id,
            password: message.password,
        })
    }
}
