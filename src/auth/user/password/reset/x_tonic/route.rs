use crate::auth::user::password::reset::y_protobuf::service::{
    request_reset_token_pb_server::RequestResetTokenPbServer,
    reset_password_pb_server::ResetPasswordPbServer,
};

use crate::auth::user::password::reset::{
    request_token::api::x_tonic::route::ServiceRequestToken,
    reset::api::x_tonic::route::ServiceReset,
};

pub struct ResetServer;

impl ResetServer {
    pub fn request_token(&self) -> RequestResetTokenPbServer<ServiceRequestToken> {
        RequestResetTokenPbServer::new(ServiceRequestToken)
    }
    pub fn reset(&self) -> ResetPasswordPbServer<ServiceReset> {
        ResetPasswordPbServer::new(ServiceReset)
    }
}
