use tonic::Request;

use crate::auth::auth_ticket::_common::y_protobuf::service::{
    validate_api_token_pb_client::ValidateApiTokenPbClient, ValidateApiTokenRequestPb,
};

use crate::auth::_api::x_outside_feature::feature::AuthOutsideService;

use crate::auth::_api::service::helper::{infra_error, set_metadata};

use super::super::infra::ValidateService;

use crate::auth::{
    _api::service::data::AuthServiceError,
    auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
    auth_user::_common::kernel::data::{AuthUserExtract, AuthUserId, RequireAuthRoles},
};

pub struct TonicValidateService<'a> {
    auth_service_url: &'static str,
    request_id: &'a str,
}

impl<'a> TonicValidateService<'a> {
    pub const fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            auth_service_url: service.auth_service_url,
            request_id,
        }
    }
}

#[async_trait::async_trait]
impl<'a> ValidateService for TonicValidateService<'a> {
    async fn validate(
        &self,
        nonce: Option<AuthNonce>,
        token: Option<AuthToken>,
        require_roles: RequireAuthRoles,
    ) -> Result<AuthUserId, AuthServiceError> {
        let mut client = ValidateApiTokenPbClient::connect(self.auth_service_url)
            .await
            .map_err(infra_error)?;

        let request: ValidateApiTokenRequestPb = require_roles.into();
        let mut request = Request::new(request);
        set_metadata(&mut request, self.request_id, nonce, token)?;

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

    use crate::auth::{
        _api::service::data::AuthServiceError,
        auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
        auth_user::_common::kernel::data::{AuthUserId, RequireAuthRoles},
    };

    pub enum StaticValidateService {
        Valid(AuthUserId),
    }

    #[async_trait::async_trait]
    impl ValidateService for StaticValidateService {
        async fn validate(
            &self,
            _nonce: Option<AuthNonce>,
            _token: Option<AuthToken>,
            _require_roles: RequireAuthRoles,
        ) -> Result<AuthUserId, AuthServiceError> {
            match self {
                Self::Valid(user_id) => Ok(user_id.clone()),
            }
        }
    }
}
