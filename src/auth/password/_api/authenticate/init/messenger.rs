use prost::Message;

use crate::auth::password::_api::y_protobuf::api::{
    AuthenticatePasswordErrorKindPb, AuthenticatePasswordErrorPb, AuthenticatePasswordPb,
    AuthenticatePasswordResultPb,
};

use crate::z_details::_api::message::helper::{
    decode_base64, encode_protobuf_base64, invalid_protobuf,
};

use crate::auth::password::_api::authenticate::infra::{
    AuthenticatePasswordFieldsExtract, AuthenticatePasswordMessenger,
};

use crate::auth::password::_api::authenticate::data::AuthenticatePasswordResponse;
use crate::z_details::_api::message::data::MessageError;

pub struct ProtobufAuthenticatePasswordMessenger {
    body: String,
}

impl ProtobufAuthenticatePasswordMessenger {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl AuthenticatePasswordMessenger for ProtobufAuthenticatePasswordMessenger {
    fn decode(&self) -> Result<AuthenticatePasswordFieldsExtract, MessageError> {
        let message = AuthenticatePasswordPb::decode(decode_base64(self.body.clone())?)
            .map_err(invalid_protobuf)?;

        Ok(AuthenticatePasswordFieldsExtract {
            login_id: message.login_id,
            password: message.password,
        })
    }
    fn encode_password_not_found(&self) -> Result<AuthenticatePasswordResponse, MessageError> {
        encode_failed(
            AuthenticatePasswordErrorKindPb::InvalidPassword,
            AuthenticatePasswordResponse::PasswordNotFound,
        )
    }
    fn encode_password_not_matched(&self) -> Result<AuthenticatePasswordResponse, MessageError> {
        encode_failed(
            AuthenticatePasswordErrorKindPb::InvalidPassword,
            AuthenticatePasswordResponse::PasswordNotMatched,
        )
    }
}

fn encode_failed(
    error_kind: AuthenticatePasswordErrorKindPb,
    response: impl Fn(String) -> AuthenticatePasswordResponse,
) -> Result<AuthenticatePasswordResponse, MessageError> {
    let message = encode_protobuf_base64(AuthenticatePasswordResultPb {
        success: false,
        err: Some(AuthenticatePasswordErrorPb {
            kind: error_kind as i32,
        }),
        ..Default::default()
    })?;
    Ok(response(message))
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::_api::authenticate::infra::{
        AuthenticatePasswordFieldsExtract, AuthenticatePasswordMessenger,
    };

    use crate::auth::password::_api::authenticate::data::AuthenticatePasswordResponse;
    use crate::z_details::_api::message::data::MessageError;

    pub struct StaticAuthenticatePasswordMessenger {
        fields: AuthenticatePasswordFieldsExtract,
    }

    impl StaticAuthenticatePasswordMessenger {
        pub const fn new(fields: AuthenticatePasswordFieldsExtract) -> Self {
            Self { fields }
        }
    }

    impl AuthenticatePasswordMessenger for StaticAuthenticatePasswordMessenger {
        fn decode(&self) -> Result<AuthenticatePasswordFieldsExtract, MessageError> {
            Ok(self.fields.clone())
        }
        fn encode_password_not_found(&self) -> Result<AuthenticatePasswordResponse, MessageError> {
            Ok(AuthenticatePasswordResponse::PasswordNotFound(
                "encoded".into(),
            ))
        }
        fn encode_password_not_matched(
            &self,
        ) -> Result<AuthenticatePasswordResponse, MessageError> {
            Ok(AuthenticatePasswordResponse::PasswordNotMatched(
                "encoded".into(),
            ))
        }
    }
}
