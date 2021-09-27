use crate::auth::auth_ticket::remote::{
    kernel::infra::{AuthClock, DiscardAuthTicketRepository},
    validate::infra::ValidateAuthTokenInfra,
};

pub trait DiscardAuthTicketInfra {
    type ValidateInfra: ValidateAuthTokenInfra;
    type Clock: AuthClock;
    type TicketRepository: DiscardAuthTicketRepository;

    fn validate_infra(&self) -> &Self::ValidateInfra;
    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
}
