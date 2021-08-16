use crate::auth::auth_ticket::_auth::kernel::infra::{AuthClock, AuthTicketRepository};

pub trait DiscardAuthTicketInfra {
    type Clock: AuthClock;
    type TicketRepository: AuthTicketRepository;

    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
}
