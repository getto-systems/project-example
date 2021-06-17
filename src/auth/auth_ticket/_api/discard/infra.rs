pub mod id_generator;

use super::super::kernel::infra::{AuthClock, AuthTicketIdGenerator, AuthTicketRepository};

pub trait DiscardAuthTicketInfra {
    type Clock: AuthClock;
    type TicketRepository: AuthTicketRepository;

    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
}
