use crate::auth::password::_api::y_protobuf::api::{
    AuthenticatePasswordErrorKindPb, AuthenticatePasswordErrorPb, AuthenticatePasswordResultPb,
};

use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::auth::password::_api::authenticate::infra::{
    AuthenticatePasswordResponse, AuthenticatePasswordResponseEncoder,
};

use crate::{
    auth::{
        auth_ticket::_api::kernel::data::AuthTokenMessage,
        password::_api::authenticate::data::{
            AuthenticatePasswordMessageEncoded, AuthenticatePasswordResult,
        },
    },
    z_details::_api::message::data::MessageError,
};

pub struct ProstAuthenticatePasswordResponseEncoder;

impl AuthenticatePasswordResponseEncoder for ProstAuthenticatePasswordResponseEncoder {
    fn encode(
        &self,
        response: AuthenticatePasswordResponse,
    ) -> Result<AuthenticatePasswordMessageEncoded, MessageError> {
        match response {
            AuthenticatePasswordResponse::InvalidPassword => {
                let message = AuthenticatePasswordResultPb {
                    success: false,
                    err: Some(AuthenticatePasswordErrorPb {
                        kind: AuthenticatePasswordErrorKindPb::InvalidPassword as i32,
                    }),
                    ..Default::default()
                };
                Ok(AuthenticatePasswordResult::InvalidPassword(
                    encode_protobuf_base64(message)?,
                ))
            }
            AuthenticatePasswordResponse::Success(ticket) => {
                let message = AuthenticatePasswordResultPb {
                    success: true,
                    value: Some(ticket.user.into()),
                    ..Default::default()
                };
                Ok(AuthenticatePasswordResult::Success(AuthTokenMessage {
                    body: encode_protobuf_base64(message)?,
                    token: ticket.token,
                }))
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::_api::authenticate::infra::{
        AuthenticatePasswordResponse, AuthenticatePasswordResponseEncoder,
    };

    use crate::{
        auth::{
            auth_ticket::_api::kernel::data::AuthTokenMessage,
            password::_api::authenticate::data::{
                AuthenticatePasswordMessageEncoded, AuthenticatePasswordResult,
            },
        },
        z_details::_api::message::data::MessageError,
    };

    pub struct StaticAuthenticatePasswordResponseEncoder;

    impl AuthenticatePasswordResponseEncoder for StaticAuthenticatePasswordResponseEncoder {
        fn encode(
            &self,
            response: AuthenticatePasswordResponse,
        ) -> Result<AuthenticatePasswordMessageEncoded, MessageError> {
            match response {
                AuthenticatePasswordResponse::InvalidPassword => Ok(
                    AuthenticatePasswordResult::InvalidPassword("INVALID-PASSWORD".into()),
                ),
                AuthenticatePasswordResponse::Success(ticket) => {
                    Ok(AuthenticatePasswordResult::Success(AuthTokenMessage {
                        body: "ENCODED".into(),
                        token: ticket.token,
                    }))
                }
            }
        }
    }
}
