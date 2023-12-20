use crate::auth::ticket::{
    authenticate::x_tonic::route::ServiceAuthenticateWithToken,
    authorize::x_tonic::route::ServiceAuthorize, logout::x_tonic::route::ServiceLogout,
};

#[derive(Default)]
pub struct AuthTicketServer {
    pub logout: ServiceLogout,
    pub authenticate_with_token: ServiceAuthenticateWithToken,
    pub authorize: ServiceAuthorize,
}
