use crate::auth::user::{
    account::x_tonic::route::AccountServer, login_id::x_tonic::route::LoginIdServer,
    password::x_tonic::route::AuthPasswordServer,
};

#[derive(Default)]
pub struct AuthUserServer {
    pub account: AccountServer,
    pub login_id: LoginIdServer,
    pub password: AuthPasswordServer,
}
