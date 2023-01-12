use crate::auth::user::password::reset::{
    request_token::y_protobuf::service::request_reset_token_pb_server::RequestResetTokenPbServer,
    reset::y_protobuf::service::reset_password_pb_server::ResetPasswordPbServer,
};

use crate::auth::user::password::reset::{
    request_token::x_tonic::route::ServiceRequestToken, reset::x_tonic::route::ServiceReset,
    token_destination::x_tonic::route::TokenDestinationServer,
};

pub struct AuthPasswordResetServer {
    pub token_destination: TokenDestinationServer,
}

impl AuthPasswordResetServer {
    pub const fn new() -> Self {
        Self {
            token_destination: TokenDestinationServer,
        }
    }
    pub fn request_token(&self) -> RequestResetTokenPbServer<ServiceRequestToken> {
        RequestResetTokenPbServer::new(ServiceRequestToken)
    }
    pub fn reset(&self) -> ResetPasswordPbServer<ServiceReset> {
        ResetPasswordPbServer::new(ServiceReset)
    }
}
