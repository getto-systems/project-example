use crate::auth::_api::y_protobuf::api::{
    AuthenticatePasswordResult_pb, AuthenticatePasswordResult_pb_Error,
    AuthenticatePasswordResult_pb_ErrorType, AuthenticatePassword_pb,
};

use crate::z_details::_api::message::helper::{decode_protobuf_base64, encode_protobuf_base64};

use super::{AuthenticateMessenger, AuthenticatePasswordFieldsExtract};

use crate::z_details::_api::message::data::MessageError;

pub struct ProtobufAuthenticateMessenger {
    body: String,
}

impl ProtobufAuthenticateMessenger {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl AuthenticateMessenger for ProtobufAuthenticateMessenger {
    fn decode(&self) -> Result<AuthenticatePasswordFieldsExtract, MessageError> {
        let message: AuthenticatePassword_pb = decode_protobuf_base64(self.body.clone())?;

        Ok(AuthenticatePasswordFieldsExtract {
            login_id: message.login_id,
            password: message.password,
        })
    }
    fn encode_invalid_password(&self) -> Result<String, MessageError> {
        let mut message = AuthenticatePasswordResult_pb::new();

        let mut err = AuthenticatePasswordResult_pb_Error::new();
        err.set_field_type(AuthenticatePasswordResult_pb_ErrorType::INVALID_PASSWORD);

        message.set_success(false);
        message.set_err(err);

        encode_protobuf_base64(message)
    }
}

#[cfg(test)]
pub mod test {
    use super::super::{AuthenticateMessenger, AuthenticatePasswordFieldsExtract};

    use crate::z_details::_api::message::data::MessageError;

    pub struct StaticAuthenticateMessenger {
        fields: AuthenticatePasswordFieldsExtract,
    }

    impl StaticAuthenticateMessenger {
        pub const fn new(fields: AuthenticatePasswordFieldsExtract) -> Self {
            Self { fields }
        }
    }

    impl AuthenticateMessenger for StaticAuthenticateMessenger {
        fn decode(&self) -> Result<AuthenticatePasswordFieldsExtract, MessageError> {
            Ok(self.fields.clone())
        }
        fn encode_invalid_password(&self) -> Result<String, MessageError> {
            Ok("encoded".into())
        }
    }
}
