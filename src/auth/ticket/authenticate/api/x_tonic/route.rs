use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::ticket::authenticate::y_protobuf::service::{
    authenticate_with_token_pb_server::AuthenticateWithTokenPb, AuthenticateWithTokenRequestPb,
    AuthenticateWithTokenResponsePb,
};

use crate::x_outside_feature::auth::{feature::AuthTonicRequest, logger::AuthLogger};

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

use crate::auth::ticket::authenticate::init::ActiveAuthenticateWithTokenMaterial;

pub struct ServiceAuthenticateWithToken;

#[async_trait::async_trait]
impl AuthenticateWithTokenPb for ServiceAuthenticateWithToken {
    async fn call(
        &self,
        request: Request<AuthenticateWithTokenRequestPb>,
    ) -> Result<Response<AuthenticateWithTokenResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request_id,
            ..
        } = AuthTonicRequest::from_request(request);

        let mut action = ActiveAuthenticateWithTokenMaterial::action(&feature);
        let logger = AuthLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(&metadata).await).respond_to()
    }
}
