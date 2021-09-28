use tonic::Request;

use crate::auth::ticket::_common::y_protobuf::service::{
    validate_api_token_pb_client::ValidateApiTokenPbClient, ValidateApiTokenRequestPb,
};

use crate::auth::remote::x_outside_feature::common::feature::AuthOutsideService;

use crate::z_lib::remote::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{
    auth::remote::service::helper::{infra_error, set_metadata},
    z_lib::remote::service::helper::new_endpoint,
};

use super::super::infra::ValidateService;
use crate::{
    auth::ticket::remote::kernel::infra::AuthMetadataContent,
    z_lib::remote::service::infra::ServiceAuthorizer,
};

use crate::auth::{
    remote::service::data::AuthServiceError,
    user::remote::kernel::data::{AuthUserExtract, AuthUserId, RequireAuthRoles},
};

pub struct TonicValidateService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer,
}

impl<'a> TonicValidateService<'a> {
    pub fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(service.service_url),
        }
    }
}

#[async_trait::async_trait]
impl<'a> ValidateService for TonicValidateService<'a> {
    async fn validate(
        &self,
        metadata: AuthMetadataContent,
        require_roles: RequireAuthRoles,
    ) -> Result<AuthUserId, AuthServiceError> {
        let mut client = ValidateApiTokenPbClient::new(
            new_endpoint(self.service_url)
                .map_err(infra_error)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let request: ValidateApiTokenRequestPb = require_roles.into();
        let mut request = Request::new(request);
        set_metadata(
            &mut request,
            self.request_id,
            self.authorizer.fetch_token().await.map_err(infra_error)?,
            metadata,
        )
        .map_err(infra_error)?;

        let response = client
            .validate(request)
            .await
            .map_err(AuthServiceError::from)?
            .into_inner();

        let user: Option<AuthUserExtract> = response.user.map(|user| user.into());
        user.ok_or(AuthServiceError::InfraError(
            "failed to decode response".into(),
        ))
        .map(|user| user.restore().into_user_id())
    }
}

#[cfg(test)]
pub mod test {
    use super::super::super::infra::ValidateService;
    use crate::auth::ticket::remote::kernel::infra::AuthMetadataContent;

    use crate::auth::{
        remote::service::data::AuthServiceError,
        user::remote::kernel::data::{AuthUserId, RequireAuthRoles},
    };

    pub struct StaticValidateService {
        user_id: AuthUserId,
    }
    impl StaticValidateService {
        pub fn new(user_id: String) -> Self {
            Self {
                user_id: AuthUserId::restore(user_id),
            }
        }
    }

    #[async_trait::async_trait]
    impl ValidateService for StaticValidateService {
        async fn validate(
            &self,
            _metadata: AuthMetadataContent,
            _require_roles: RequireAuthRoles,
        ) -> Result<AuthUserId, AuthServiceError> {
            Ok(self.user_id.clone())
        }
    }
}
