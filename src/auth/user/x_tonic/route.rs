use crate::auth::user::{
    account::x_tonic::route::AccountServer, login_id::x_tonic::route::LoginIdServer,
    password::x_tonic::route::AuthPasswordServer,
};

pub struct AuthUserServer {
    pub account: AccountServer,
    pub login_id: LoginIdServer,
    pub password: AuthPasswordServer,
}

impl AuthUserServer {
    pub const fn new() -> Self {
        Self {
            account: AccountServer,
            login_id: LoginIdServer,
            password: AuthPasswordServer::new(),
        }
    }
}
