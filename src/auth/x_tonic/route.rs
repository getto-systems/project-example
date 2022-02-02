use crate::auth::{ticket::x_tonic::route::AuthTicketServer, user::x_tonic::route::AuthUserServer};

pub struct AuthServer {
    pub ticket: AuthTicketServer,
    pub user: AuthUserServer,
}

impl AuthServer {
    pub const fn new() -> Self {
        Self {
            ticket: AuthTicketServer,
            user: AuthUserServer::new(),
        }
    }
}
