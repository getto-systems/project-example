use uuid::Uuid;

use crate::auth::auth_ticket::remote::issue::infra::AuthTicketIdGenerator;

use crate::auth::auth_ticket::_auth::kernel::data::AuthTicketId;

pub struct UuidAuthTicketIdGenerator;

impl UuidAuthTicketIdGenerator {
    pub const fn new() -> Self {
        Self
    }
}

impl AuthTicketIdGenerator for UuidAuthTicketIdGenerator {
    fn generate(&self) -> AuthTicketId {
        AuthTicketId::new(Uuid::new_v4().to_string())
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::remote::issue::infra::AuthTicketIdGenerator;

    use crate::auth::auth_ticket::_auth::kernel::data::AuthTicketId;

    pub struct StaticAuthTicketIdGenerator {
        ticket_id: AuthTicketId,
    }

    impl StaticAuthTicketIdGenerator {
        pub const fn new(ticket_id: AuthTicketId) -> Self {
            Self { ticket_id }
        }
    }

    impl AuthTicketIdGenerator for StaticAuthTicketIdGenerator {
        fn generate(&self) -> AuthTicketId {
            self.ticket_id.clone()
        }
    }
}
