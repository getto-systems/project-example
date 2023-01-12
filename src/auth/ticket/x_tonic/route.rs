use crate::auth::ticket::{
    authenticate::y_protobuf::service::authenticate_with_token_pb_server::AuthenticateWithTokenPbServer,
    authorize::y_protobuf::service::clarify_authorize_token_pb_server::ClarifyAuthorizeTokenPbServer,
    logout::y_protobuf::service::logout_pb_server::LogoutPbServer,
};

use crate::auth::ticket::{
    authenticate::x_tonic::route::ServiceAuthenticateWithToken,
    authorize::x_tonic::route::ServiceClarifyAuthorizeToken, logout::x_tonic::route::ServiceLogout,
};

pub struct AuthTicketServer;

impl AuthTicketServer {
    pub fn logout(&self) -> LogoutPbServer<ServiceLogout> {
        LogoutPbServer::new(ServiceLogout)
    }
    pub fn authenticate_with_token(
        &self,
    ) -> AuthenticateWithTokenPbServer<ServiceAuthenticateWithToken> {
        AuthenticateWithTokenPbServer::new(ServiceAuthenticateWithToken)
    }
    pub fn clarify_authorize_token(
        &self,
    ) -> ClarifyAuthorizeTokenPbServer<ServiceClarifyAuthorizeToken> {
        ClarifyAuthorizeTokenPbServer::new(ServiceClarifyAuthorizeToken)
    }
}
