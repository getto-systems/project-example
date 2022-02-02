use crate::auth::ticket::remote::y_protobuf::service::{
    check_auth_ticket_pb_server::CheckAuthTicketPbServer, logout_pb_server::LogoutPbServer,
    validate_api_token_pb_server::ValidateApiTokenPbServer,
};

use crate::auth::ticket::{
    check::remote::x_tonic::route::ServiceCheck, logout::remote::x_tonic::route::ServiceLogout,
    validate::x_tonic::route::ServiceValidate,
};

pub struct AuthTicketServer;

impl AuthTicketServer {
    pub fn logout(&self) -> LogoutPbServer<ServiceLogout> {
        LogoutPbServer::new(ServiceLogout)
    }
    pub fn check(&self) -> CheckAuthTicketPbServer<ServiceCheck> {
        CheckAuthTicketPbServer::new(ServiceCheck)
    }
    pub fn validate(&self) -> ValidateApiTokenPbServer<ServiceValidate> {
        ValidateApiTokenPbServer::new(ServiceValidate)
    }
}