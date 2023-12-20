mod detail;

use std::sync::Arc;

use crate::auth::{
    kernel::infra::AuthClock,
    ticket::issue::infra::{
        AuthTicketIdGenerator, IssueAuthTicketInfra, IssueAuthTicketLogger,
        IssueAuthTicketRepository,
    },
};

use crate::auth::ticket::{
    issue::data::IssueAuthTicketError,
    kernel::data::{AuthTicket, AuthenticateSuccess},
};

pub struct IssueAuthTicketAction<M: IssueAuthTicketInfra> {
    infra: M,
    logger: Arc<dyn IssueAuthTicketLogger>,
}

impl<M: IssueAuthTicketInfra> IssueAuthTicketAction<M> {
    pub async fn issue(
        &self,
        auth: AuthenticateSuccess,
    ) -> Result<AuthTicket, IssueAuthTicketError> {
        let ticket = AuthTicket {
            ticket_id: self.infra.ticket_id_generator().generate(),
            attrs: auth.into(),
        };

        let issued_at = self.infra.clock().now();
        let limit = self.logger.calculate_expansion_limit(
            issued_at.expansion_limit(&self.infra.config().authenticate_expansion_limit),
        );

        self.infra
            .repository()
            .register(ticket.clone(), limit, issued_at)
            .await
            .map_err(|err| self.logger.failed_to_register_ticket(err))?;

        Ok(self.logger.succeed_to_issue_ticket(ticket))
    }
}
