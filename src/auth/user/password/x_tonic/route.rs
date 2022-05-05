use crate::auth::user::password::{
    authenticate::y_protobuf::service::authenticate_password_pb_server::AuthenticatePasswordPbServer,
    change::y_protobuf::service::{
        change_password_pb_server::ChangePasswordPbServer,
        overwrite_password_pb_server::OverwritePasswordPbServer,
    },
};

use crate::auth::user::password::{
    authenticate::x_tonic::route::ServiceAuthenticate,
    change::x_tonic::route::{ServiceChangePassword, ServiceOverwritePassword},
    reset::x_tonic::route::ResetServer,
};

pub struct PasswordServer {
    pub reset: ResetServer,
}

impl PasswordServer {
    pub const fn new() -> Self {
        Self {
            reset: ResetServer::new(),
        }
    }
    pub fn authenticate(&self) -> AuthenticatePasswordPbServer<ServiceAuthenticate> {
        AuthenticatePasswordPbServer::new(ServiceAuthenticate)
    }
    pub fn change(&self) -> ChangePasswordPbServer<ServiceChangePassword> {
        ChangePasswordPbServer::new(ServiceChangePassword)
    }
    pub fn overwrite(&self) -> OverwritePasswordPbServer<ServiceOverwritePassword> {
        OverwritePasswordPbServer::new(ServiceOverwritePassword)
    }
}
