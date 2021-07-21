use crate::auth::{
    auth_ticket::_api::y_protobuf::api::AuthenticateResponsePb,
    password::_api::y_protobuf::api::AuthenticatePasswordResultPb,
};

use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::auth::password::_api::authenticate::infra::{
    AuthenticatePasswordResponse, AuthenticatePasswordResponseEncoder,
};

use crate::{
    auth::{
        auth_ticket::_api::kernel::data::AuthTokenMessageEncoded,
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
                    ..Default::default()
                };
                Ok(AuthenticatePasswordResult::InvalidPassword(
                    encode_protobuf_base64(message)?,
                ))
            }
            AuthenticatePasswordResponse::Success(response) => {
                let (user, token) = response.extract();
                let message: AuthenticateResponsePb = user.into();
                Ok(AuthenticatePasswordResult::Success(
                    AuthTokenMessageEncoded {
                        message: encode_protobuf_base64(message)?,
                        token,
                    },
                ))
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
            auth_ticket::_api::kernel::data::AuthTokenMessageEncoded,
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
                AuthenticatePasswordResponse::Success(response) => {
                    let (_user, token) = response.extract();
                    Ok(AuthenticatePasswordResult::Success(
                        AuthTokenMessageEncoded {
                            message: "ENCODED".into(),
                            token,
                        },
                    ))
                }
            }
        }
    }
}
