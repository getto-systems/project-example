use crate::auth::{ticket::x_tonic::route::AuthTicketServer, user::x_tonic::route::AuthUserServer};

#[derive(Default)]
pub struct AuthServer {
    pub ticket: AuthTicketServer,
    pub user: AuthUserServer,
}
