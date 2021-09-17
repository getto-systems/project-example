use tonic::Request;

use crate::auth::password::_common::y_protobuf::service::{
    change_password_pb_client::ChangePasswordPbClient, ChangePasswordRequestPb,
};

use crate::auth::_api::x_outside_feature::feature::AuthOutsideService;

use crate::z_details::_api::service::init::authorizer::GoogleServiceAuthorizer;

use crate::auth::_api::service::helper::{
    infra_error, new_endpoint, set_authorization, set_request_id,
};

use crate::auth::password::{
    _api::change::infra::{ChangePasswordResponse, ChangePasswordService},
    _common::change::infra::ChangePasswordFieldsExtract,
};

use crate::auth::{
    _api::service::data::AuthServiceError, auth_user::_common::kernel::data::AuthUserId,
};

pub struct TonicChangePasswordService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer,
}

impl<'a> TonicChangePasswordService<'a> {
    pub fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(service.service_url),
        }
    }
}

#[async_trait::async_trait]
impl<'a> ChangePasswordService for TonicChangePasswordService<'a> {
    async fn change(
        &self,
        user_id: AuthUserId,
        fields: ChangePasswordFieldsExtract,
    ) -> Result<ChangePasswordResponse, AuthServiceError> {
        let mut client = ChangePasswordPbClient::new(
            new_endpoint(self.service_url)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let mut request = Request::new(ChangePasswordRequestPb {
            user_id: user_id.extract(),
            current_password: fields.current_password,
            new_password: fields.new_password,
        });
        set_authorization(&mut request, &self.authorizer).await?;
        set_request_id(&mut request, self.request_id)?;

        let response = client
            .change(request)
            .await
            .map_err(AuthServiceError::from)?;
        let response: Option<ChangePasswordResponse> = response.into_inner().into();
        response.ok_or(AuthServiceError::InfraError(
            "failed to decode response".into(),
        ))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::{
        auth_user::_common::kernel::data::AuthUserId,
        password::{
            _api::change::infra::{ChangePasswordResponse, ChangePasswordService},
            _common::change::infra::ChangePasswordFieldsExtract,
        },
    };

    use crate::auth::_api::service::data::AuthServiceError;

    pub struct StaticChangePasswordService;

    #[async_trait::async_trait]
    impl ChangePasswordService for StaticChangePasswordService {
        async fn change(
            &self,
            _user_id: AuthUserId,
            _fields: ChangePasswordFieldsExtract,
        ) -> Result<ChangePasswordResponse, AuthServiceError> {
            Ok(ChangePasswordResponse::Success)
        }
    }
}
