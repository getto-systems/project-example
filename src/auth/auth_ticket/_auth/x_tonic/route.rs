use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::z_details::_common::{logger::Logger, response::tonic::RespondTo};

use crate::auth::auth_ticket::_common::y_protobuf::service::{
    logout_pb_server::{LogoutPb, LogoutPbServer},
    renew_auth_ticket_pb_server::{RenewAuthTicketPb, RenewAuthTicketPbServer},
    LogoutRequestPb, LogoutResponsePb, RenewAuthTicketRequestPb, RenewAuthTicketResponsePb,
};

use crate::x_outside_feature::_auth::{
    feature::{extract_request, TonicRequest},
    logger::app_logger,
};

use crate::auth::auth_ticket::_auth::{
    action_logout::init::LogoutFeature, action_renew::init::RenewAuthTicketFeature,
};

pub struct AuthTicketServer;

impl AuthTicketServer {
    pub fn logout(&self) -> LogoutPbServer<Logout> {
        LogoutPbServer::new(Logout)
    }
    pub fn renew(&self) -> RenewAuthTicketPbServer<Renew> {
        RenewAuthTicketPbServer::new(Renew)
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

        let logger = app_logger("auth.auth_ticket.logout", &metadata);
        let mut action = LogoutFeature::action(&data.auth, &metadata);
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

        let logger = app_logger("auth.auth_ticket.renew", &metadata);
        let mut action = RenewAuthTicketFeature::action(&data.auth, &metadata);
        action.subscribe(move |state| logger.log(state.log_level(), state));

        flatten(action.ignite().await).respond_to()
    }
}
