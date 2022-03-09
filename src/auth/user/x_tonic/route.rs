use crate::auth::user::{
    account::x_tonic::route::AccountServer, login_id::x_tonic::route::LoginIdServer,
    password::x_tonic::route::PasswordServer,
};

pub struct AuthUserServer {
    pub account: AccountServer,
    pub login_id: LoginIdServer,
    pub password: PasswordServer,
}

impl AuthUserServer {
    pub const fn new() -> Self {
        Self {
            account: AccountServer,
            login_id: LoginIdServer,
            password: PasswordServer::new(),
        }
    }
}
