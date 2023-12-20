mod detail;

use std::sync::Arc;

use crate::auth::ticket::logout::infra::{LogoutInfra, LogoutLogger, LogoutRepository};

use crate::auth::ticket::{
    authenticate::data::CheckAuthenticateTokenSuccess,
    logout::data::{LogoutError, LogoutSuccess},
};

pub struct LogoutAction<M: LogoutInfra> {
    infra: M,
    logger: Arc<dyn LogoutLogger>,
}

impl<M: LogoutInfra> LogoutAction<M> {
    pub async fn logout(
        &self,
        auth: CheckAuthenticateTokenSuccess,
    ) -> Result<LogoutSuccess, LogoutError> {
        self.logger.try_to_logout();

        let ticket = auth.extract();

        self.infra
            .repository()
            .discard(&ticket)
            .await
            .map_err(|err| self.logger.failed_to_discard_ticket(err))?;

        Ok(self.logger.succeed_to_logout(LogoutSuccess::new(ticket)))
    }
}
