use crate::auth::_api::y_protobuf::api::{
    AuthenticatePasswordResult_pb, AuthenticatePasswordResult_pb_Error,
    AuthenticatePasswordResult_pb_ErrorType, AuthenticatePassword_pb,
};

use crate::z_details::_api::message::helper::{decode_protobuf_base64, encode_protobuf_base64};

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
        let message: AuthenticatePassword_pb = decode_protobuf_base64(self.body.clone())?;

        Ok(AuthenticatePasswordFieldsExtract {
            login_id: message.login_id,
            password: message.password,
        })
    }
    fn encode_password_not_found(&self) -> Result<AuthenticatePasswordResponse, MessageError> {
        encode_failed(
            AuthenticatePasswordResult_pb_ErrorType::INVALID_PASSWORD,
            AuthenticatePasswordResponse::PasswordNotFound,
        )
    }
    fn encode_password_not_matched(&self) -> Result<AuthenticatePasswordResponse, MessageError> {
        encode_failed(
            AuthenticatePasswordResult_pb_ErrorType::INVALID_PASSWORD,
            AuthenticatePasswordResponse::PasswordNotMatched,
        )
    }
}

fn encode_failed(
    field_type: AuthenticatePasswordResult_pb_ErrorType,
    response: impl Fn(String) -> AuthenticatePasswordResponse,
) -> Result<AuthenticatePasswordResponse, MessageError> {
    let mut message = AuthenticatePasswordResult_pb::new();
    message.set_success(false);

    let mut err = AuthenticatePasswordResult_pb_Error::new();
    err.set_field_type(field_type);
    message.set_err(err);

    let message = encode_protobuf_base64(message)?;
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
            Ok(AuthenticatePasswordResponse::PasswordNotFound("encoded".into()))
        }
        fn encode_password_not_matched(&self) -> Result<AuthenticatePasswordResponse, MessageError> {
            Ok(AuthenticatePasswordResponse::PasswordNotMatched("encoded".into()))
        }
    }
}
