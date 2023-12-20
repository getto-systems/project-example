mod detail;

use std::sync::Arc;

use crate::auth::{
    kernel::infra::AuthClock,
    ticket::encode::infra::{
        AuthenticateTokenEncoder, AuthorizeTokenEncoder, CdnTokenEncoder, EncodeAuthTokenInfra,
        EncodeAuthTokenLogger, EncodeAuthTokenRepository,
    },
};

use crate::auth::ticket::{
    encode::data::{AuthTokenExpires, EncodeAuthTokenError, EncodeAuthTokenSuccess},
    kernel::data::{AuthTicket, AuthToken},
};

pub struct EncodeAuthTokenAction<M: EncodeAuthTokenInfra> {
    logger: Arc<dyn EncodeAuthTokenLogger>,
    infra: M,
}

impl<M: EncodeAuthTokenInfra> EncodeAuthTokenAction<M> {
    pub async fn encode(
        &self,
        ticket: AuthTicket,
    ) -> Result<EncodeAuthTokenSuccess, EncodeAuthTokenError> {
        self.logger.try_to_encode_auth_token();

        let limit = self
            .infra
            .repository()
            .lookup_expansion_limit(&ticket)
            .await
            .map_err(|err| self.logger.failed_to_lookup_expansion_limit(err))?
            .ok_or_else(|| {
                self.logger
                    .expansion_limit_not_found(EncodeAuthTokenError::TicketNotFound)
            })?;

        let expires = self.logger.calculate_token_expires(AuthTokenExpires {
            authenticate: self
                .infra
                .clock()
                .now()
                .expires_with_limit(&self.infra.config().authenticate_expires, &limit),

            authorize: self
                .infra
                .clock()
                .now()
                .expires_with_limit(&self.infra.config().authorize_expires, &limit),

            cdn: self
                .infra
                .clock()
                .now()
                .expires_with_limit(&self.infra.config().cdn_expires, &limit),
        });

        let token = AuthToken {
            authenticate_token: self
                .infra
                .authenticate_encoder()
                .encode(ticket.clone(), expires.authenticate)
                .map_err(|err| self.logger.failed_to_encode_token(err))?,

            authorize_token: self
                .infra
                .authorize_encoder()
                .encode(ticket.clone(), expires.authorize)
                .map_err(|err| self.logger.failed_to_encode_token(err))?,

            cdn_token: self
                .infra
                .cdn_encoder()
                .encode(expires.cdn)
                .map_err(|err| self.logger.failed_to_encode_token(err))?,
        };

        Ok(self
            .logger
            .succeed_to_encode_auth_token(EncodeAuthTokenSuccess::new(token, ticket.attrs.granted)))
    }
}
