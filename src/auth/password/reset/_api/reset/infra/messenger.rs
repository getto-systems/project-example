use crate::auth::_api::y_protobuf::api::{
    ResetPasswordResult_pb, ResetPasswordResult_pb_Error, ResetPasswordResult_pb_ErrorType,
    ResetPassword_pb,
};

use crate::z_details::_api::message::helper::{decode_protobuf_base64, encode_protobuf_base64};

use crate::auth::password::reset::_api::reset::infra::{
    ResetPasswordFieldsExtract, ResetPasswordMessenger,
};

use crate::auth::password::reset::_api::reset::data::ResetPasswordResponse;
use crate::z_details::_api::message::data::MessageError;

pub struct ProtobufResetPasswordMessenger {
    body: String,
}

impl ProtobufResetPasswordMessenger {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl ResetPasswordMessenger for ProtobufResetPasswordMessenger {
    fn decode(&self) -> Result<ResetPasswordFieldsExtract, MessageError> {
        let message: ResetPassword_pb = decode_protobuf_base64(self.body.clone())?;

        Ok(ResetPasswordFieldsExtract {
            login_id: message.login_id,
            password: message.password,
            reset_token: message.reset_token,
        })
    }
    fn encode_invalid_reset(&self) -> Result<ResetPasswordResponse, MessageError> {
        encode(ResetPasswordResult_pb_ErrorType::INVALID_RESET)
    }
    fn encode_already_reset(&self) -> Result<ResetPasswordResponse, MessageError> {
        encode(ResetPasswordResult_pb_ErrorType::ALREADY_RESET)
    }
}

fn encode(
    field_type: ResetPasswordResult_pb_ErrorType,
) -> Result<ResetPasswordResponse, MessageError> {
    let mut message = ResetPasswordResult_pb::new();
    message.set_success(false);

    let mut err = ResetPasswordResult_pb_Error::new();
    err.set_field_type(field_type);
    message.set_err(err);

    let message = encode_protobuf_base64(message)?;
    Ok(ResetPasswordResponse { message })
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::reset::_api::reset::infra::{
        ResetPasswordFieldsExtract, ResetPasswordMessenger,
    };

    use crate::auth::password::reset::_api::reset::data::ResetPasswordResponse;
    use crate::z_details::_api::message::data::MessageError;

    pub struct StaticResetPasswordMessenger {
        fields: ResetPasswordFieldsExtract,
    }

    impl StaticResetPasswordMessenger {
        pub const fn new(fields: ResetPasswordFieldsExtract) -> Self {
            Self { fields }
        }
    }

    impl ResetPasswordMessenger for StaticResetPasswordMessenger {
        fn decode(&self) -> Result<ResetPasswordFieldsExtract, MessageError> {
            Ok(self.fields.clone())
        }
        fn encode_invalid_reset(&self) -> Result<ResetPasswordResponse, MessageError> {
            Ok(ResetPasswordResponse {
                message: "encoded".into(),
            })
        }
        fn encode_already_reset(&self) -> Result<ResetPasswordResponse, MessageError> {
            Ok(ResetPasswordResponse {
                message: "encoded".into(),
            })
        }
    }
}
