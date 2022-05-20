use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::password::reset::request_token::y_protobuf::service::{
    request_reset_token_pb_server::RequestResetTokenPb, RequestResetTokenRequestPb,
    RequestResetTokenResponsePb,
};

use crate::x_outside_feature::auth::{
    feature::{extract_auth_request, AuthTonicRequest},
    logger::app_logger,
};

use crate::x_content::metadata::metadata_request_id;

use crate::auth::user::password::reset::request_token::init::RequestResetTokenStruct;

use crate::z_lib::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceRequestToken;

impl ServiceRequestToken {
    pub const fn name() -> &'static str {
        "auth.user.password.reset.request_token"
    }
}

#[async_trait::async_trait]
impl RequestResetTokenPb for ServiceRequestToken {
    async fn request_token(
        &self,
        request: Request<RequestResetTokenRequestPb>,
    ) -> Result<Response<RequestResetTokenResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request,
        } = extract_auth_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = RequestResetTokenStruct::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
