use crate::auth::ticket::{
    logout::y_protobuf::service::logout_pb_server::LogoutPbServer,
    y_protobuf::service::{
        check_auth_ticket_pb_server::CheckAuthTicketPbServer,
        validate_api_token_pb_server::ValidateApiTokenPbServer,
    },
};

use crate::auth::ticket::{
    check::api::x_tonic::route::ServiceCheck, logout::api::x_tonic::route::ServiceLogout,
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
