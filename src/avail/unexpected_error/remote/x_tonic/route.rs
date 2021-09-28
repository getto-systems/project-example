use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::z_details::_common::{logger::Logger, response::tonic::RespondTo};

use crate::avail::unexpected_error::_common::y_protobuf::service::{
    notify_pb_server::{NotifyPb, NotifyPbServer},
    NotifyRequestPb, NotifyResponsePb,
};

use crate::avail::unexpected_error::remote::action_notify::init::NotifyUnexpectedErrorFeature;

use crate::x_outside_feature::remote::{
    common::metadata::metadata_request_id,
    example::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
};

pub struct UnexpectedErrorServer;

impl UnexpectedErrorServer {
    pub fn notify(&self) -> NotifyPbServer<Notify> {
        NotifyPbServer::new(Notify)
    }
}

pub struct Notify;

#[async_trait::async_trait]
impl NotifyPb for Notify {
    async fn notify(
        &self,
        request: Request<NotifyRequestPb>,
    ) -> Result<Response<NotifyResponsePb>, Status> {
        let TonicRequest {
            metadata,
            data,
            request,
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);
        let logger = app_logger("avail.unexpected_error.notify", request_id.into());

        let mut action = NotifyUnexpectedErrorFeature::action(&data, &request_id, &metadata);
        action.subscribe(move |state| logger.log(state.log_level(), state));

        let request_decoder = NotifyUnexpectedErrorFeature::request_decoder(request);
        flatten(action.ignite(request_decoder).await).respond_to()
    }
}
