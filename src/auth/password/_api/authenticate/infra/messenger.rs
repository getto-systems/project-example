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
    fn encode_invalid_password(&self) -> Result<AuthenticatePasswordResponse, MessageError> {
        let mut message = AuthenticatePasswordResult_pb::new();
        message.set_success(false);

        let mut err = AuthenticatePasswordResult_pb_Error::new();
        err.set_field_type(AuthenticatePasswordResult_pb_ErrorType::INVALID_PASSWORD);
        message.set_err(err);

        let message = encode_protobuf_base64(message)?;

        Ok(AuthenticatePasswordResponse { message })
    }
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
        fn encode_invalid_password(&self) -> Result<AuthenticatePasswordResponse, MessageError> {
            Ok(AuthenticatePasswordResponse {
                message: "encoded".into(),
            })
        }
    }
}
