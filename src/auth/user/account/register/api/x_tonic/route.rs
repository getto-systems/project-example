use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::account::register::y_protobuf::service::{
    register_auth_user_account_pb_server::RegisterAuthUserAccountPb,
    RegisterAuthUserAccountRequestPb, RegisterAuthUserAccountResponsePb,
};

use crate::x_outside_feature::auth::{
    feature::{extract_auth_request, AuthTonicRequest},
    logger::app_logger,
};

use crate::x_content::metadata::metadata_request_id;

use crate::auth::user::account::register::init::RegisterAuthUserAccountFeature;

use crate::z_lib::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceRegisterUser;

impl ServiceRegisterUser {
    pub const fn name() -> &'static str {
        "auth.user.account.register"
    }
}

#[async_trait::async_trait]
impl RegisterAuthUserAccountPb for ServiceRegisterUser {
    async fn register_user(
        &self,
        request: Request<RegisterAuthUserAccountRequestPb>,
    ) -> Result<Response<RegisterAuthUserAccountResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request,
        } = extract_auth_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = RegisterAuthUserAccountFeature::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
