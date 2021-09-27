use crate::auth::{
    ticket::remote::x_tonic::route::AuthTicketServer,
    password::remote::x_tonic::route::PasswordServer,
};

pub struct AuthServer {
    pub auth_ticket: AuthTicketServer,
    pub password: PasswordServer,
}

impl AuthServer {
    pub const fn new() -> Self {
        Self {
            auth_ticket: AuthTicketServer,
            password: PasswordServer::new(),
        }
    }
}
