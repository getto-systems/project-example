mod detail;

use std::sync::Arc;

use crate::auth::{
    kernel::infra::AuthClock,
    ticket::authorize::infra::{
        AuthorizeFieldsExtract, AuthorizeInfra, AuthorizeLogger, AuthorizeRepository,
        AuthorizeTokenDecoder, CheckAuthorizeTokenInfra, CheckAuthorizeTokenLogger,
    },
};

use crate::auth::ticket::{
    authorize::data::{
        AuthorizeError, AuthorizeSuccess, CheckAuthorizeTokenError, CheckAuthorizeTokenSuccess,
    },
    kernel::data::{AuthPermissionRequired, AuthorizeTokenExtract},
};

pub struct CheckAuthorizeTokenAction<M: CheckAuthorizeTokenInfra> {
    logger: Arc<dyn CheckAuthorizeTokenLogger>,
    infra: M,
}

impl<M: CheckAuthorizeTokenInfra> CheckAuthorizeTokenAction<M> {
    pub async fn check(
        &self,
        token: impl AuthorizeTokenExtract,
        required: AuthPermissionRequired,
    ) -> Result<CheckAuthorizeTokenSuccess, CheckAuthorizeTokenError> {
        self.logger.try_to_check_authorize_token();

        let token = token
            .convert()
            .map_err(|err| self.logger.invalid_request(err))?;

        let ticket = self
            .infra
            .token_decoder()
            .decode(token.clone())
            .map_err(|err| self.logger.invalid_token(err))?;

        ticket
            .attrs
            .granted
            .has_enough_permission(&required)
            .map_err(|err| self.logger.forbidden(err))?;

        Ok(CheckAuthorizeTokenSuccess::new(
            self.logger
                .succeed_to_check_authorize_token(ticket.attrs.granted),
        ))
    }
}

pub struct AuthorizeAction<M: AuthorizeInfra> {
    logger: Arc<dyn AuthorizeLogger>,
    infra: M,
}

impl<M: AuthorizeInfra> AuthorizeAction<M> {
    pub async fn authorize(
        &self,
        fields: impl AuthorizeFieldsExtract,
    ) -> Result<AuthorizeSuccess, AuthorizeError> {
        self.logger.try_to_authorize();

        let fields = fields
            .convert()
            .map_err(|err| self.logger.invalid_request(err))?;

        let ticket = self
            .infra
            .token_decoder()
            .decode(fields.token.clone())
            .map_err(|err| self.logger.invalid_token(err))?;

        ticket
            .attrs
            .granted
            .has_enough_permission(&fields.required)
            .map_err(|err| self.logger.forbidden(err))?;

        let expansion_limit = self
            .infra
            .repository()
            .lookup_expansion_limit(&ticket)
            .await
            .map_err(|err| self.logger.failed_to_lookup_expansion_limit(err))?
            .ok_or_else(|| {
                self.logger
                    .expansion_limit_not_found(AuthorizeError::TicketNotFound)
            })?;

        if expansion_limit.has_elapsed(&self.infra.clock().now()) {
            return Err(self
                .logger
                .ticket_has_expired(AuthorizeError::TicketHasExpired));
        }

        let granted = self
            .infra
            .repository()
            .lookup_permission_granted(&ticket.attrs.user_id)
            .await
            .map_err(|err| self.logger.failed_to_lookup_permission_granted(err))?
            .ok_or_else(|| {
                self.logger
                    .permission_granted_not_found(AuthorizeError::UserNotFound)
            })?;

        granted
            .has_enough_permission(&fields.required)
            .map_err(|err| self.logger.forbidden(err))?;

        Ok(self.logger.authorized(AuthorizeSuccess::new(ticket.attrs)))
    }
}
