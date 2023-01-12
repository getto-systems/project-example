use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::password::authenticate::y_protobuf::service::{
    authenticate_with_password_pb_server::AuthenticateWithPasswordPb,
    AuthenticateWithPasswordRequestPb, AuthenticateWithPasswordResponsePb,
};

use crate::x_outside_feature::auth::{feature::AuthTonicRequest, logger::AuthLogger};

use crate::auth::user::password::authenticate::init::ActiveAuthenticateWithPasswordMaterial;

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceAuthenticateWithPassword;

#[async_trait::async_trait]
impl AuthenticateWithPasswordPb for ServiceAuthenticateWithPassword {
    async fn authenticate(
        &self,
        request: Request<AuthenticateWithPasswordRequestPb>,
    ) -> Result<Response<AuthenticateWithPasswordResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            request,
            request_id,
            ..
        } = AuthTonicRequest::from_request(request);

        let mut action = ActiveAuthenticateWithPasswordMaterial::action(&feature);
        let logger = AuthLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(request).await).respond_to()
    }
}
