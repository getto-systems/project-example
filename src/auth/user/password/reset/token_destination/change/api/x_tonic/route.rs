use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::password::reset::token_destination::change::y_protobuf::service::{
    change_reset_token_destination_pb_server::ChangeResetTokenDestinationPb,
    ChangeResetTokenDestinationRequestPb, ChangeResetTokenDestinationResponsePb,
};

use crate::x_outside_feature::auth::{feature::AuthTonicRequest, logger::AuthLogger};

use crate::auth::user::password::reset::token_destination::change::init::ActiveChangeResetTokenDestinationMaterial;

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceChangeDestination;

#[async_trait::async_trait]
impl ChangeResetTokenDestinationPb for ServiceChangeDestination {
    async fn change_destination(
        &self,
        request: Request<ChangeResetTokenDestinationRequestPb>,
    ) -> Result<Response<ChangeResetTokenDestinationResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request,
            request_id,
        } = AuthTonicRequest::from_request(request);

        let mut action =
            ActiveChangeResetTokenDestinationMaterial::action(&feature, request_id.clone());
        let logger = AuthLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(&metadata, request).await).respond_to()
    }
}
