use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::ticket::authorize::y_protobuf::service::{
    clarify_authorize_token_pb_server::ClarifyAuthorizeTokenPb, ClarifyAuthorizeTokenRequestPb,
    ClarifyAuthorizeTokenResponsePb,
};

use crate::x_outside_feature::auth::{feature::AuthTonicRequest, logger::AuthLogger};

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

use crate::auth::ticket::authorize::init::ActiveClarifyAuthorizeTokenMaterial;

pub struct ServiceClarifyAuthorizeToken;

#[async_trait::async_trait]
impl ClarifyAuthorizeTokenPb for ServiceClarifyAuthorizeToken {
    async fn clarify(
        &self,
        request: Request<ClarifyAuthorizeTokenRequestPb>,
    ) -> Result<Response<ClarifyAuthorizeTokenResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request,
            request_id,
        } = AuthTonicRequest::from_request(request);

        let mut action = ActiveClarifyAuthorizeTokenMaterial::action(&feature);
        let logger = AuthLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite((&metadata, request)).await).respond_to()
    }
}
