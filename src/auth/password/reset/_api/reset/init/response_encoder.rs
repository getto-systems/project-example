use crate::auth::password::reset::_api::y_protobuf::api::{
    ResetPasswordErrorKindPb, ResetPasswordErrorPb, ResetPasswordResultPb,
};

use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::auth::password::reset::_api::reset::infra::{
    ResetPasswordResponse, ResetPasswordResponseEncoder,
};

use crate::{
    auth::{
        auth_ticket::_api::kernel::data::AuthTokenMessageEncoded,
        password::reset::_api::reset::data::{ResetPasswordMessageEncoded, ResetPasswordResult},
    },
    z_details::_api::message::data::MessageError,
};

pub struct ProstResetPasswordResponseEncoder;

impl ResetPasswordResponseEncoder for ProstResetPasswordResponseEncoder {
    fn encode(
        &self,
        response: ResetPasswordResponse,
    ) -> Result<ResetPasswordMessageEncoded, MessageError> {
        match response {
            ResetPasswordResponse::InvalidReset => {
                let message = ResetPasswordResultPb {
                    success: false,
                    err: Some(ResetPasswordErrorPb {
                        kind: ResetPasswordErrorKindPb::InvalidReset as i32,
                    }),
                    ..Default::default()
                };
                Ok(ResetPasswordResult::InvalidReset(encode_protobuf_base64(
                    message,
                )?))
            }
            ResetPasswordResponse::AlreadyReset => {
                let message = ResetPasswordResultPb {
                    success: false,
                    err: Some(ResetPasswordErrorPb {
                        kind: ResetPasswordErrorKindPb::AlreadyReset as i32,
                    }),
                    ..Default::default()
                };
                Ok(ResetPasswordResult::AlreadyReset(encode_protobuf_base64(
                    message,
                )?))
            }
            ResetPasswordResponse::Success(ticket) => {
                let message = ResetPasswordResultPb {
                    success: true,
                    value: Some(ticket.user.into()),
                    ..Default::default()
                };
                Ok(ResetPasswordResult::Success(AuthTokenMessageEncoded {
                    message: encode_protobuf_base64(message)?,
                    token: ticket.token,
                }))
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::reset::_api::reset::infra::{
        ResetPasswordResponse, ResetPasswordResponseEncoder,
    };

    use crate::{
        auth::{
            auth_ticket::_api::kernel::data::AuthTokenMessageEncoded,
            password::reset::_api::reset::data::{
                ResetPasswordMessageEncoded, ResetPasswordResult,
            },
        },
        z_details::_api::message::data::MessageError,
    };

    pub struct StaticResetPasswordResponseEncoder;

    impl ResetPasswordResponseEncoder for StaticResetPasswordResponseEncoder {
        fn encode(
            &self,
            response: ResetPasswordResponse,
        ) -> Result<ResetPasswordMessageEncoded, MessageError> {
            match response {
                ResetPasswordResponse::InvalidReset => {
                    Ok(ResetPasswordResult::InvalidReset("INVALID-RESET".into()))
                }
                ResetPasswordResponse::AlreadyReset => {
                    Ok(ResetPasswordResult::AlreadyReset("ALREADY-RESET".into()))
                }
                ResetPasswordResponse::Success(ticket) => {
                    Ok(ResetPasswordResult::Success(AuthTokenMessageEncoded {
                        message: "ENCODED".into(),
                        token: ticket.token,
                    }))
                }
            }
        }
    }
}
