use crate::auth::password::_api::y_protobuf::api::{
    AuthenticatePasswordErrorKindPb, ChangePasswordErrorPb, ChangePasswordResultPb,
};

use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::auth::password::_api::change::infra::{
    ChangePasswordResponse, ChangePasswordResponseEncoder,
};

use crate::{
    auth::password::_api::change::data::ChangePasswordResult,
    z_details::_api::message::data::MessageError,
};

pub struct ProstChangePasswordResponseEncoder;

impl ChangePasswordResponseEncoder for ProstChangePasswordResponseEncoder {
    fn encode(
        &self,
        response: ChangePasswordResponse,
    ) -> Result<ChangePasswordResult, MessageError> {
        match response {
            ChangePasswordResponse::InvalidPassword => {
                let message = ChangePasswordResultPb {
                    success: false,
                    err: Some(ChangePasswordErrorPb {
                        kind: AuthenticatePasswordErrorKindPb::InvalidPassword as i32,
                    }),
                    ..Default::default()
                };
                Ok(ChangePasswordResult::InvalidPassword(
                    encode_protobuf_base64(message)?,
                ))
            }
            ChangePasswordResponse::Success => {
                let message = ChangePasswordResultPb {
                    success: true,
                    ..Default::default()
                };
                Ok(ChangePasswordResult::Success(encode_protobuf_base64(
                    message,
                )?))
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::_api::change::infra::{
        ChangePasswordResponse, ChangePasswordResponseEncoder,
    };

    use crate::{
        auth::password::_api::change::data::ChangePasswordResult,
        z_details::_api::message::data::MessageError,
    };

    pub struct StaticChangePasswordResponseEncoder;

    impl ChangePasswordResponseEncoder for StaticChangePasswordResponseEncoder {
        fn encode(
            &self,
            response: ChangePasswordResponse,
        ) -> Result<ChangePasswordResult, MessageError> {
            match response {
                ChangePasswordResponse::InvalidPassword => Ok(
                    ChangePasswordResult::InvalidPassword("INVALID-PASSWORD".into()),
                ),
                ChangePasswordResponse::Success => {
                    Ok(ChangePasswordResult::Success("ENCODED".into()))
                }
            }
        }
    }
}
