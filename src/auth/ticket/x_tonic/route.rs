use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::ticket::check::remote::x_tonic::route::ServiceCheck;
use crate::z_lib::remote::{logger::Logger, response::tonic::RespondTo};

use crate::auth::ticket::remote::y_protobuf::service::{
    check_auth_ticket_pb_server::CheckAuthTicketPbServer,
    logout_pb_server::{LogoutPb, LogoutPbServer},
    validate_api_token_pb_server::{ValidateApiTokenPb, ValidateApiTokenPbServer},
    LogoutRequestPb, LogoutResponsePb, ValidateApiTokenRequestPb, ValidateApiTokenResponsePb,
};

use crate::x_outside_feature::remote::{
    auth::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
    common::metadata::metadata_request_id,
};

use crate::auth::ticket::remote::{
    logout::init::LogoutStruct, validate::init::ValidateApiTokenStruct,
};

pub struct AuthTicketServer;

impl AuthTicketServer {
    pub fn logout(&self) -> LogoutPbServer<Logout> {
        LogoutPbServer::new(Logout)
    }
    pub fn check(&self) -> CheckAuthTicketPbServer<ServiceCheck> {
        CheckAuthTicketPbServer::new(ServiceCheck)
    }
    pub fn validate(&self) -> ValidateApiTokenPbServer<Validate> {
        ValidateApiTokenPbServer::new(Validate)
    }
}

pub struct Logout;

#[async_trait::async_trait]
impl LogoutPb for Logout {
    async fn logout(
        &self,
        request: Request<LogoutRequestPb>,
    ) -> Result<Response<LogoutResponsePb>, Status> {
        let TonicRequest {
            feature, metadata, ..
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger("auth.ticket.logout", request_id.into());
        let mut action = LogoutStruct::action(&feature, &metadata);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}

pub struct Validate;

#[async_trait::async_trait]
impl ValidateApiTokenPb for Validate {
    async fn validate(
        &self,
        request: Request<ValidateApiTokenRequestPb>,
    ) -> Result<Response<ValidateApiTokenResponsePb>, Status> {
        let TonicRequest {
            feature,
            metadata,
            request,
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger("auth.ticket.validate", request_id.into());
        let mut action = ValidateApiTokenStruct::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
