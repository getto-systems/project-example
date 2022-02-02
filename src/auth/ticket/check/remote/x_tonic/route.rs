use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::tonic::RespondTo};

use crate::auth::ticket::remote::y_protobuf::service::{
    check_auth_ticket_pb_server::CheckAuthTicketPb, CheckAuthTicketRequestPb,
    CheckAuthTicketResponsePb,
};

use crate::x_outside_feature::remote::{
    auth::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
    common::metadata::metadata_request_id,
};

use crate::auth::ticket::check::remote::init::CheckAuthTicketStruct;

pub struct ServiceCheck;

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

        let logger = app_logger("auth.ticket.check", request_id.into());
        let mut action = CheckAuthTicketStruct::action(&feature, &metadata);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
