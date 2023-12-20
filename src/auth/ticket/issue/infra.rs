use crate::auth::kernel::infra::AuthClock;

use crate::{
    auth::{
        kernel::data::{AuthDateTime, ExpansionLimitDateTime, ExpansionLimitDuration},
        ticket::kernel::data::{AuthTicket, AuthTicketId},
    },
    common::api::repository::data::RepositoryError,
};

pub trait IssueAuthTicketInfra {
    type Clock: AuthClock;
    type Repository: IssueAuthTicketRepository;
    type TicketIdGenerator: AuthTicketIdGenerator;

    fn clock(&self) -> &Self::Clock;
    fn repository(&self) -> &Self::Repository;
    fn ticket_id_generator(&self) -> &Self::TicketIdGenerator;
    fn config(&self) -> &IssueAuthTicketConfig;
}

#[async_trait::async_trait]
pub trait IssueAuthTicketRepository {
    async fn register(
        &self,
        ticket: AuthTicket,
        limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) -> Result<(), RepositoryError>;
}

pub trait AuthTicketIdGenerator {
    fn generate(&self) -> AuthTicketId;
}

#[derive(Clone)]
pub struct IssueAuthTicketConfig {
    pub authenticate_expansion_limit: ExpansionLimitDuration,
}

pub trait IssueAuthTicketLogger: Send + Sync {
    fn try_to_issue_ticket(&self);
    fn calculate_expansion_limit(&self, limit: ExpansionLimitDateTime) -> ExpansionLimitDateTime;
    fn failed_to_register_ticket(&self, err: RepositoryError) -> RepositoryError;
    fn succeed_to_issue_ticket(&self, ticket: AuthTicket) -> AuthTicket;
}
