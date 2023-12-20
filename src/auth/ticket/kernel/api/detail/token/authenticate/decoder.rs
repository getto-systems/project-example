use std::sync::Arc;

use actix_web::web::Data;
use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey};

use crate::x_outside_feature::{auth::feature::AuthAppFeature, proxy::feature::ProxyAppFeature};

use crate::{
    auth::ticket::kernel::detail::token::authenticate::data::Claims, common::api::feature::AsInfra,
    x_content::permission::AuthPermission,
};

use crate::auth::ticket::authenticate::infra::AuthenticateTokenDecoder;

use crate::auth::{
    ticket::kernel::data::{
        AuthPermissionGranted, AuthTicket, AuthTicketAttrs, AuthTicketId, AuthenticateToken,
        DecodeAuthenticateTokenError,
    },
    user::kernel::data::AuthUserId,
};

pub struct JwtAuthenticateTokenDecoder {
    key: Arc<DecodingKey>,
}

impl AsInfra<JwtAuthenticateTokenDecoder> for Data<ProxyAppFeature> {
    fn as_infra(&self) -> JwtAuthenticateTokenDecoder {
        JwtAuthenticateTokenDecoder {
            key: Arc::clone(&self.auth.decoding_key.authenticate),
        }
    }
}

impl AsInfra<JwtAuthenticateTokenDecoder> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> JwtAuthenticateTokenDecoder {
        JwtAuthenticateTokenDecoder {
            key: Arc::clone(&self.decoding_key.authenticate),
        }
    }
}

impl AuthenticateTokenDecoder for JwtAuthenticateTokenDecoder {
    fn decode(&self, token: AuthenticateToken) -> Result<AuthTicket, DecodeAuthenticateTokenError> {
        let validation = Claims::validation();

        let data: Claims = decode(&token.extract(), &self.key, &validation)
            .map_err(|err| match err.kind() {
                ErrorKind::ExpiredSignature => DecodeAuthenticateTokenError::Expired,
                _ => DecodeAuthenticateTokenError::Invalid(format!("{}", err)),
            })?
            .claims;

        // JWT で検証しているので restore で受け取る
        Ok(AuthTicket {
            ticket_id: AuthTicketId::restore(data.ticket_id),
            attrs: AuthTicketAttrs {
                user_id: AuthUserId::restore(data.user_id),
                granted: AuthPermissionGranted::restore(
                    data.granted
                        .into_iter()
                        .filter_map(AuthPermission::convert)
                        .collect(),
                ),
            },
        })
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use crate::auth::ticket::authenticate::infra::AuthenticateTokenDecoder;

    use crate::auth::ticket::kernel::data::{
        AuthTicket, AuthenticateToken, DecodeAuthenticateTokenError,
    };

    #[derive(Clone)]
    pub struct MockAuthenticateTokenDecoder(
        HashMap<String, Result<AuthTicket, DecodeAuthenticateTokenError>>,
    );

    impl MockAuthenticateTokenDecoder {
        pub fn new(map: Vec<(String, Result<AuthTicket, DecodeAuthenticateTokenError>)>) -> Self {
            Self(map.into_iter().collect())
        }
    }

    impl AuthenticateTokenDecoder for MockAuthenticateTokenDecoder {
        fn decode(
            &self,
            token: AuthenticateToken,
        ) -> Result<AuthTicket, DecodeAuthenticateTokenError> {
            match self.0.get(&token.extract()) {
                Some(result) => result.clone(),
                None => Err(DecodeAuthenticateTokenError::Invalid(
                    "invalid authenticate token".to_owned(),
                )),
            }
        }
    }
}
