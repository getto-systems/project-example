use uuid::Uuid;

use super::AuthTicketIdGenerator;

use super::super::super::kernel::data::AuthTicketId;

pub struct UuidAuthTicketIdGenerator {}

impl UuidAuthTicketIdGenerator {
    pub const fn new() -> Self {
        Self {}
    }
}

impl AuthTicketIdGenerator for UuidAuthTicketIdGenerator {
    fn generate(&self) -> AuthTicketId {
        AuthTicketId::new(Uuid::new_v4().to_string())
    }
}
