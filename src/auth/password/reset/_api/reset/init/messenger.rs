use prost::Message;

use crate::auth::password::reset::_api::y_protobuf::api::{
    ResetPasswordErrorKindPb, ResetPasswordErrorPb, ResetPasswordPb, ResetPasswordResultPb,
};
use crate::z_details::_api::message::helper::{
    decode_base64, encode_protobuf_base64, invalid_protobuf,
};

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
        let message =
            ResetPasswordPb::decode(decode_base64(self.body.clone())?).map_err(invalid_protobuf)?;

        Ok(ResetPasswordFieldsExtract {
            login_id: message.login_id,
            password: message.password,
            reset_token: message.reset_token,
        })
    }
    fn encode_not_found(&self) -> Result<ResetPasswordResponse, MessageError> {
        encode_failed(
            ResetPasswordErrorKindPb::InvalidReset,
            ResetPasswordResponse::NotFound,
        )
    }
    fn encode_already_reset(&self) -> Result<ResetPasswordResponse, MessageError> {
        encode_failed(
            ResetPasswordErrorKindPb::AlreadyReset,
            ResetPasswordResponse::AlreadyReset,
        )
    }
    fn encode_expired(&self) -> Result<ResetPasswordResponse, MessageError> {
        encode_failed(
            ResetPasswordErrorKindPb::InvalidReset,
            ResetPasswordResponse::Expired,
        )
    }
    fn encode_invalid_login_id(&self) -> Result<ResetPasswordResponse, MessageError> {
        encode_failed(
            ResetPasswordErrorKindPb::InvalidReset,
            ResetPasswordResponse::InvalidLoginId,
        )
    }
}

fn encode_failed(
    error_kind: ResetPasswordErrorKindPb,
    response: impl Fn(String) -> ResetPasswordResponse,
) -> Result<ResetPasswordResponse, MessageError> {
    let message = encode_protobuf_base64(ResetPasswordResultPb {
        success: false,
        err: Some(ResetPasswordErrorPb {
            kind: error_kind as i32,
        }),
        ..Default::default()
    })?;
    Ok(response(message))
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
        fn encode_not_found(&self) -> Result<ResetPasswordResponse, MessageError> {
            Ok(ResetPasswordResponse::NotFound("encoded".into()))
        }
        fn encode_already_reset(&self) -> Result<ResetPasswordResponse, MessageError> {
            Ok(ResetPasswordResponse::AlreadyReset("encoded".into()))
        }
        fn encode_expired(&self) -> Result<ResetPasswordResponse, MessageError> {
            Ok(ResetPasswordResponse::Expired("encoded".into()))
        }
        fn encode_invalid_login_id(&self) -> Result<ResetPasswordResponse, MessageError> {
            Ok(ResetPasswordResponse::InvalidLoginId("encoded".into()))
        }
    }
}
