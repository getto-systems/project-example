use crate::auth::auth_ticket::_auth::x_tonic::route::AuthTicketServer;

pub struct AuthServer {
    auth_ticket: AuthTicketServer,
}

impl AuthServer {
    pub const fn new() -> Self {
        Self {
            auth_ticket: AuthTicketServer::new(),
        }
    }

    pub const fn auth_ticket(&self) -> &AuthTicketServer {
        &self.auth_ticket
    }
}
