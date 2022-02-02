use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::tonic::RespondTo};

use crate::auth::ticket::remote::y_protobuf::service::{
    check_auth_ticket_pb_server::CheckAuthTicketPbServer,
    logout_pb_server::LogoutPbServer,
    validate_api_token_pb_server::{ValidateApiTokenPb, ValidateApiTokenPbServer},
    ValidateApiTokenRequestPb, ValidateApiTokenResponsePb,
};

use crate::x_outside_feature::remote::{
    auth::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
    common::metadata::metadata_request_id,
};

use crate::auth::ticket::{
    check::remote::x_tonic::route::ServiceCheck, logout::remote::x_tonic::route::ServiceLogout,
};

use crate::auth::ticket::remote::validate::init::ValidateApiTokenStruct;

pub struct AuthTicketServer;

impl AuthTicketServer {
    pub fn logout(&self) -> LogoutPbServer<ServiceLogout> {
        LogoutPbServer::new(ServiceLogout)
    }
    pub fn check(&self) -> CheckAuthTicketPbServer<ServiceCheck> {
        CheckAuthTicketPbServer::new(ServiceCheck)
    }
    pub fn validate(&self) -> ValidateApiTokenPbServer<Validate> {
        ValidateApiTokenPbServer::new(Validate)
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
