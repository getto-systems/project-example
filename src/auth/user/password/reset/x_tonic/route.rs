use crate::auth::user::password::reset::{
    request_token::x_tonic::route::ServiceRequestToken, reset::x_tonic::route::ServiceReset,
    token_destination::x_tonic::route::TokenDestinationServer,
};

#[derive(Default)]
pub struct AuthPasswordResetServer {
    pub request_token: ServiceRequestToken,
    pub reset: ServiceReset,
    pub token_destination: TokenDestinationServer,
}
