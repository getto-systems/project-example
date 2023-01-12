use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey};

use crate::auth::x_outside_feature::feature::AuthOutsideDecodingKey;

use crate::x_content::permission::AuthPermission;

use crate::auth::ticket::kernel::init::token::authenticate::data::Claims;

use crate::auth::ticket::authenticate::infra::AuthenticateTokenDecoder;

use crate::auth::{
    ticket::kernel::data::{
        AuthPermissionGranted, AuthTicket, AuthTicketAttrs, AuthTicketId, AuthenticateToken,
        DecodeAuthenticateTokenError,
    },
    user::kernel::data::AuthUserId,
};

pub struct JwtAuthenticateTokenDecoder<'a> {
    key: &'a DecodingKey,
}

impl<'a> JwtAuthenticateTokenDecoder<'a> {
    pub const fn new(decoding_key: &'a AuthOutsideDecodingKey) -> Self {
        Self {
            key: &decoding_key.authenticate,
        }
    }
}

impl<'a> AuthenticateTokenDecoder for JwtAuthenticateTokenDecoder<'a> {
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
    use crate::auth::ticket::authenticate::infra::AuthenticateTokenDecoder;

    use crate::auth::ticket::kernel::data::{
        AuthTicket, AuthenticateToken, DecodeAuthenticateTokenError,
    };

    pub enum StaticAuthenticateTokenDecoder {
        Valid(AuthTicket),
        Expired,
    }

    impl AuthenticateTokenDecoder for StaticAuthenticateTokenDecoder {
        fn decode(
            &self,
            _token: AuthenticateToken,
        ) -> Result<AuthTicket, DecodeAuthenticateTokenError> {
            match self {
                Self::Valid(ticket) => Ok(ticket.clone()),
                Self::Expired => Err(DecodeAuthenticateTokenError::Expired),
            }
        }
    }
}
