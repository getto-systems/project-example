use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::z_lib::api::{logger::Logger, response::tonic::RespondTo};

use crate::auth::ticket::check::y_protobuf::service::{
    check_auth_ticket_pb_server::CheckAuthTicketPb, CheckAuthTicketRequestPb,
    CheckAuthTicketResponsePb,
};

use crate::x_outside_feature::api::{
    auth::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
    common::metadata::metadata_request_id,
};

use crate::auth::ticket::check::api::init::CheckAuthTicketStruct;

pub struct ServiceCheck;

impl ServiceCheck {
    pub const fn name() -> &'static str {
        "auth.ticket.check"
    }
}

#[async_trait::async_trait]
impl CheckAuthTicketPb for ServiceCheck {
    async fn check(
        &self,
        request: Request<CheckAuthTicketRequestPb>,
    ) -> Result<Response<CheckAuthTicketResponsePb>, Status> {
        let TonicRequest {
            feature, metadata, ..
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = CheckAuthTicketStruct::action(&feature, &metadata);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
