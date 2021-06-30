use crate::auth::auth_ticket::_api::kernel::infra::AuthTicketInfra;

pub trait DiscardAuthTicketInfra {
    type TicketInfra: AuthTicketInfra;

    fn ticket_infra(&self) -> &Self::TicketInfra;
}
