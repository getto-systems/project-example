use crate::auth::auth_ticket::_auth::kernel::infra::AuthTicketInfra;

pub trait DiscardAuthTicketInfra {
    type TicketInfra: AuthTicketInfra;

    fn extract(self) -> Self::TicketInfra;
}
