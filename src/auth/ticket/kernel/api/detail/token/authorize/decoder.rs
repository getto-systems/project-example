use std::sync::Arc;

use actix_web::web::Data;
use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey};

use crate::x_outside_feature::{
    auth::feature::AuthAppFeature, core::feature::CoreAppFeature, proxy::feature::ProxyAppFeature,
};

use crate::{
    auth::ticket::kernel::detail::token::authorize::data::Claims, common::api::feature::AsInfra,
    x_content::permission::AuthPermission,
};

use crate::auth::ticket::authorize::infra::AuthorizeTokenDecoder;

use crate::auth::{
    ticket::kernel::data::{
        AuthPermissionGranted, AuthTicket, AuthTicketAttrs, AuthTicketId, AuthorizeToken,
        DecodeAuthorizeTokenError,
    },
    user::kernel::data::AuthUserId,
};

pub struct JwtAuthorizeTokenDecoder {
    key: Arc<DecodingKey>,
}

impl AsInfra<JwtAuthorizeTokenDecoder> for Data<ProxyAppFeature> {
    fn as_infra(&self) -> JwtAuthorizeTokenDecoder {
        JwtAuthorizeTokenDecoder {
            key: Arc::clone(&self.auth.decoding_key.authorize),
        }
    }
}

impl AsInfra<JwtAuthorizeTokenDecoder> for Arc<CoreAppFeature> {
    fn as_infra(&self) -> JwtAuthorizeTokenDecoder {
        JwtAuthorizeTokenDecoder {
            key: Arc::clone(&self.auth.decoding_key.key),
        }
    }
}

impl AsInfra<JwtAuthorizeTokenDecoder> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> JwtAuthorizeTokenDecoder {
        JwtAuthorizeTokenDecoder {
            key: Arc::clone(&self.decoding_key.authorize),
        }
    }
}

impl AuthorizeTokenDecoder for JwtAuthorizeTokenDecoder {
    fn decode(&self, token: AuthorizeToken) -> Result<AuthTicket, DecodeAuthorizeTokenError> {
        let validation = Claims::validation();

        let data: Claims = decode(&token.extract(), &self.key, &validation)
            .map_err(|err| match err.kind() {
                ErrorKind::ExpiredSignature => DecodeAuthorizeTokenError::Expired,
                _ => DecodeAuthorizeTokenError::Invalid(format!("{}", err)),
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

    use crate::auth::ticket::authorize::infra::AuthorizeTokenDecoder;

    use crate::auth::ticket::kernel::data::{
        AuthTicket, AuthorizeToken, DecodeAuthorizeTokenError,
    };

    #[derive(Clone)]
    pub struct MockAuthorizeTokenDecoder(
        HashMap<String, Result<AuthTicket, DecodeAuthorizeTokenError>>,
    );

    impl MockAuthorizeTokenDecoder {
        pub fn new(map: Vec<(String, Result<AuthTicket, DecodeAuthorizeTokenError>)>) -> Self {
            Self(map.into_iter().collect())
        }
    }

    impl AuthorizeTokenDecoder for MockAuthorizeTokenDecoder {
        fn decode(&self, token: AuthorizeToken) -> Result<AuthTicket, DecodeAuthorizeTokenError> {
            match self.0.get(&token.extract()) {
                Some(result) => result.clone(),
                None => Err(DecodeAuthorizeTokenError::Invalid(
                    "invalid authorize token".to_owned(),
                )),
            }
        }
    }
}
