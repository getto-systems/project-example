use crate::auth::auth_ticket::_auth::kernel::infra::{AuthClock, DiscardAuthTicketRepository};

pub trait DiscardAuthTicketInfra {
    type Clock: AuthClock;
    type TicketRepository: DiscardAuthTicketRepository;

    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
}
