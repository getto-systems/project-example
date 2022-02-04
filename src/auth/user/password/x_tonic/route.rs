use crate::auth::user::password::y_protobuf::service::{
    authenticate_password_pb_server::AuthenticatePasswordPbServer,
    change_password_pb_server::ChangePasswordPbServer,
};

use crate::auth::user::password::{
    authenticate::remote::x_tonic::route::ServiceAuthenticate,
    change::remote::x_tonic::route::ServiceChange, reset::x_tonic::route::ResetServer,
};

pub struct PasswordServer {
    pub reset: ResetServer,
}

impl PasswordServer {
    pub const fn new() -> Self {
        Self { reset: ResetServer }
    }
    pub fn authenticate(&self) -> AuthenticatePasswordPbServer<ServiceAuthenticate> {
        AuthenticatePasswordPbServer::new(ServiceAuthenticate)
    }
    pub fn change(&self) -> ChangePasswordPbServer<ServiceChange> {
        ChangePasswordPbServer::new(ServiceChange)
    }
}
