use prost::Message;

use crate::auth::password::_api::y_protobuf::api::AuthenticatePasswordPb;

use crate::z_details::_api::message::helper::{decode_base64, invalid_protobuf};

use crate::auth::password::_api::authenticate::infra::{
    AuthenticatePasswordFieldsExtract, AuthenticatePasswordRequestDecoder,
};

use crate::z_details::_api::message::data::MessageError;

pub struct ProstAuthenticatePasswordRequestDecoder {
    body: String,
}

impl ProstAuthenticatePasswordRequestDecoder {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl AuthenticatePasswordRequestDecoder for ProstAuthenticatePasswordRequestDecoder {
    fn decode(&self) -> Result<AuthenticatePasswordFieldsExtract, MessageError> {
        // TODO body は clone したくない
        let message = AuthenticatePasswordPb::decode(decode_base64(self.body.clone())?)
            .map_err(invalid_protobuf)?;

        Ok(AuthenticatePasswordFieldsExtract {
            login_id: message.login_id,
            password: message.password,
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::_api::authenticate::infra::{
        AuthenticatePasswordFieldsExtract, AuthenticatePasswordRequestDecoder,
    };

    use crate::z_details::_api::message::data::MessageError;

    pub struct StaticAuthenticatePasswordRequestDecoder {
        pub fields: AuthenticatePasswordFieldsExtract,
    }

    impl AuthenticatePasswordRequestDecoder for StaticAuthenticatePasswordRequestDecoder {
        fn decode(&self) -> Result<AuthenticatePasswordFieldsExtract, MessageError> {
            // TODO self を consume する api にすれば clone しなくて良くなる
            Ok(self.fields.clone())
        }
    }
}
