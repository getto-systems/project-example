use crate::auth::user::{
    account::remote::x_tonic::route::AccountServer,
    password::remote::x_tonic::route::PasswordServer,
};

pub struct AuthUserServer {
    pub account: AccountServer,
    pub password: PasswordServer,
}

impl AuthUserServer {
    pub const fn new() -> Self {
        Self {
            account: AccountServer,
            password: PasswordServer::new(),
        }
    }
}
