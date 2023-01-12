use crate::auth::user::password::{
    authenticate::y_protobuf::service::authenticate_with_password_pb_server::AuthenticateWithPasswordPbServer,
    change::y_protobuf::service::{
        change_password_pb_server::ChangePasswordPbServer,
        overwrite_password_pb_server::OverwritePasswordPbServer,
    },
};

use crate::auth::user::password::{
    authenticate::x_tonic::route::ServiceAuthenticateWithPassword,
    change::x_tonic::route::{ServiceChangePassword, ServiceOverwritePassword},
    reset::x_tonic::route::AuthPasswordResetServer,
};

pub struct AuthPasswordServer {
    pub reset: AuthPasswordResetServer,
}

impl AuthPasswordServer {
    pub const fn new() -> Self {
        Self {
            reset: AuthPasswordResetServer::new(),
        }
    }
    pub fn authenticate(
        &self,
    ) -> AuthenticateWithPasswordPbServer<ServiceAuthenticateWithPassword> {
        AuthenticateWithPasswordPbServer::new(ServiceAuthenticateWithPassword)
    }
    pub fn change(&self) -> ChangePasswordPbServer<ServiceChangePassword> {
        ChangePasswordPbServer::new(ServiceChangePassword)
    }
    pub fn overwrite(&self) -> OverwritePasswordPbServer<ServiceOverwritePassword> {
        OverwritePasswordPbServer::new(ServiceOverwritePassword)
    }
}
