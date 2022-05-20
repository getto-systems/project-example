use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::password::reset::token_destination::change::y_protobuf::service::{
    change_reset_token_destination_pb_server::ChangeResetTokenDestinationPb,
    ChangeResetTokenDestinationRequestPb, ChangeResetTokenDestinationResponsePb,
};

use crate::x_outside_feature::auth::{
    feature::{extract_auth_request, AuthTonicRequest},
    logger::app_logger,
};

use crate::x_content::metadata::metadata_request_id;

use crate::auth::user::password::reset::token_destination::change::init::ChangeResetTokenDestinationFeature;

use crate::z_lib::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceChangeDestination;

impl ServiceChangeDestination {
    pub const fn name() -> &'static str {
        "auth.user.password.reset.token-destination.change"
    }
}

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
        } = extract_auth_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = ChangeResetTokenDestinationFeature::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
