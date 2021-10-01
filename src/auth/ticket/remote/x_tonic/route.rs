use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::tonic::RespondTo};

use crate::auth::ticket::remote::y_protobuf::service::{
    logout_pb_server::{LogoutPb, LogoutPbServer},
    renew_auth_ticket_pb_server::{RenewAuthTicketPb, RenewAuthTicketPbServer},
    validate_api_token_pb_server::{ValidateApiTokenPb, ValidateApiTokenPbServer},
    LogoutRequestPb, LogoutResponsePb, RenewAuthTicketRequestPb, RenewAuthTicketResponsePb,
    ValidateApiTokenRequestPb, ValidateApiTokenResponsePb,
};

use crate::x_outside_feature::remote::{
    auth::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
    common::metadata::metadata_request_id,
};

use crate::auth::ticket::remote::{
    action_logout::init::LogoutFeature, action_renew::init::RenewAuthTicketFeature,
    action_validate::init::ValidateApiTokenFeature,
};

pub struct AuthTicketServer;

impl AuthTicketServer {
    pub fn logout(&self) -> LogoutPbServer<Logout> {
        LogoutPbServer::new(Logout)
    }
    pub fn renew(&self) -> RenewAuthTicketPbServer<Renew> {
        RenewAuthTicketPbServer::new(Renew)
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
        let TonicRequest { data, metadata, .. } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger("auth.auth_ticket.logout", request_id.into());
        let mut action = LogoutFeature::action(&data, &metadata);
        action.subscribe(move |state| logger.log(state.log_level(), state));

        flatten(action.ignite().await).respond_to()
    }
}

pub struct Renew;

#[async_trait::async_trait]
impl RenewAuthTicketPb for Renew {
    async fn renew(
        &self,
        request: Request<RenewAuthTicketRequestPb>,
    ) -> Result<Response<RenewAuthTicketResponsePb>, Status> {
        let TonicRequest { data, metadata, .. } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger("auth.auth_ticket.renew", request_id.into());
        let mut action = RenewAuthTicketFeature::action(&data, &metadata);
        action.subscribe(move |state| logger.log(state.log_level(), state));

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
            data,
            metadata,
            request,
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger("auth.auth_ticket.renew", request_id.into());
        let mut action = ValidateApiTokenFeature::action(&data, &metadata);
        action.subscribe(move |state| logger.log(state.log_level(), state));

        let request_decoder = ValidateApiTokenFeature::request_decoder(request);
        flatten(action.ignite(request_decoder).await).respond_to()
    }
}