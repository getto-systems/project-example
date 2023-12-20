mod detail;

use std::sync::Arc;

use crate::auth::ticket::authenticate::infra::{
    AuthenticateTokenDecoder, CheckAuthenticateTokenInfra, CheckAuthenticateTokenLogger,
};

use crate::auth::ticket::{
    authenticate::data::{CheckAuthenticateTokenError, CheckAuthenticateTokenSuccess},
    kernel::data::AuthenticateTokenExtract,
};

pub struct CheckAuthenticateTokenAction<M: CheckAuthenticateTokenInfra> {
    infra: M,
    logger: Arc<dyn CheckAuthenticateTokenLogger>,
}

impl<M: CheckAuthenticateTokenInfra> CheckAuthenticateTokenAction<M> {
    pub fn check(
        &self,
        token: impl AuthenticateTokenExtract,
    ) -> Result<CheckAuthenticateTokenSuccess, CheckAuthenticateTokenError> {
        let token = token
            .convert()
            .map_err(|err| self.logger.invalid_request(err))?;

        let ticket = self
            .infra
            .token_decoder()
            .decode(token)
            .map_err(|err| self.logger.invalid_token(err))?;

        Ok(self
            .logger
            .succeed_to_check_authenticate_token(CheckAuthenticateTokenSuccess::new(ticket)))
    }
}
