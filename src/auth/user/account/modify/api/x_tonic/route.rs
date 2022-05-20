use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::account::modify::y_protobuf::service::{
    modify_auth_user_account_pb_server::ModifyAuthUserAccountPb, ModifyAuthUserAccountRequestPb,
    ModifyAuthUserAccountResponsePb,
};

use crate::x_outside_feature::auth::{
    feature::{extract_auth_request, AuthTonicRequest},
    logger::app_logger,
};

use crate::x_content::metadata::metadata_request_id;

use crate::auth::user::account::modify::init::ModifyAuthUserAccountFeature;

use crate::z_lib::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceModifyUser;

impl ServiceModifyUser {
    pub const fn name() -> &'static str {
        "auth.user.account.modify"
    }
}

#[async_trait::async_trait]
impl ModifyAuthUserAccountPb for ServiceModifyUser {
    async fn modify_user(
        &self,
        request: Request<ModifyAuthUserAccountRequestPb>,
    ) -> Result<Response<ModifyAuthUserAccountResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request,
        } = extract_auth_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = ModifyAuthUserAccountFeature::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
