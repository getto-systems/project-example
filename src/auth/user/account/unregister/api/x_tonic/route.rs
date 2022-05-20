use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::account::unregister::y_protobuf::service::{
    unregister_auth_user_account_pb_server::UnregisterAuthUserAccountPb,
    UnregisterAuthUserAccountRequestPb, UnregisterAuthUserAccountResponsePb,
};

use crate::x_outside_feature::auth::{
    feature::{extract_auth_request, AuthTonicRequest},
    logger::app_logger,
};

use crate::x_content::metadata::metadata_request_id;

use crate::auth::user::account::unregister::init::UnregisterAuthUserAccountFeature;

use crate::z_lib::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceUnregisterUser;

impl ServiceUnregisterUser {
    pub const fn name() -> &'static str {
        "auth.user.account.unregister"
    }
}

#[async_trait::async_trait]
impl UnregisterAuthUserAccountPb for ServiceUnregisterUser {
    async fn unregister_user(
        &self,
        request: Request<UnregisterAuthUserAccountRequestPb>,
    ) -> Result<Response<UnregisterAuthUserAccountResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request,
        } = extract_auth_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = UnregisterAuthUserAccountFeature::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
