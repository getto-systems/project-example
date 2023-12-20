use crate::auth::user::password::{
    authenticate::x_tonic::route::ServiceAuthenticateWithPassword,
    change::x_tonic::route::{ServiceChangePassword, ServiceOverwritePassword},
    reset::x_tonic::route::AuthPasswordResetServer,
};

#[derive(Default)]
pub struct AuthPasswordServer {
    pub authenticate: ServiceAuthenticateWithPassword,
    pub change: ServiceChangePassword,
    pub overwrite: ServiceOverwritePassword,
    pub reset: AuthPasswordResetServer,
}
