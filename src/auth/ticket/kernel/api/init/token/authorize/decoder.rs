use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey};

use crate::auth::x_outside_feature::feature::AuthOutsideDecodingKey;

use crate::x_content::permission::AuthPermission;

use crate::auth::ticket::kernel::init::token::authorize::data::Claims;

use crate::auth::ticket::authorize::infra::AuthorizeTokenDecoder;

use crate::auth::{
    ticket::kernel::data::{
        AuthPermissionGranted, AuthTicket, AuthTicketAttrs, AuthTicketId, AuthorizeToken,
        DecodeAuthorizeTokenError,
    },
    user::kernel::data::AuthUserId,
};

pub struct JwtAuthorizeTokenDecoder<'a> {
    key: &'a DecodingKey,
}

impl<'a> JwtAuthorizeTokenDecoder<'a> {
    pub const fn new(decoding_key: &'a AuthOutsideDecodingKey) -> Self {
        Self {
            key: &decoding_key.authorize,
        }
    }
}

impl<'a> AuthorizeTokenDecoder for JwtAuthorizeTokenDecoder<'a> {
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
    use crate::auth::ticket::authorize::infra::AuthorizeTokenDecoder;

    use crate::auth::ticket::kernel::data::{
        AuthTicket, AuthorizeToken, DecodeAuthorizeTokenError,
    };

    pub enum StaticAuthorizeTokenDecoder {
        Valid(AuthTicket),
        Expired,
    }

    impl AuthorizeTokenDecoder for StaticAuthorizeTokenDecoder {
        fn decode(&self, _token: AuthorizeToken) -> Result<AuthTicket, DecodeAuthorizeTokenError> {
            match self {
                Self::Valid(ticket) => Ok(ticket.clone()),
                Self::Expired => Err(DecodeAuthorizeTokenError::Expired),
            }
        }
    }
}
