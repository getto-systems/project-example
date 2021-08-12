use tonic::Request;

use crate::auth::password::reset::_common::y_protobuf::service::{
    reset_password_pb_client::ResetPasswordPbClient, ResetPasswordRequestPb,
};

use crate::auth::_api::x_outside_feature::feature::AuthOutsideService;

use crate::auth::_api::service::helper::{infra_error, set_metadata};

use crate::auth::password::reset::{
    _api::reset::infra::{ResetPasswordResponse, ResetPasswordService},
    _common::reset::infra::ResetPasswordFieldsExtract,
};

use crate::auth::{
    _api::service::data::ServiceError,
    auth_ticket::_common::kernel::data::{AuthNonceValue, AuthTokenValue},
};

pub struct TonicResetPasswordService<'a> {
    auth_service_url: &'static str,
    request_id: &'a str,
}

impl<'a> TonicResetPasswordService<'a> {
    pub const fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            auth_service_url: service.auth_service_url,
            request_id,
        }
    }
}

#[async_trait::async_trait]
impl<'a> ResetPasswordService for TonicResetPasswordService<'a> {
    async fn reset(
        &self,
        nonce: AuthNonceValue,
        token: AuthTokenValue,
        fields: ResetPasswordFieldsExtract,
    ) -> Result<ResetPasswordResponse, ServiceError> {
        let mut client = ResetPasswordPbClient::connect(self.auth_service_url)
            .await
            .map_err(infra_error)?;

        let mut request = Request::new(ResetPasswordRequestPb {
            reset_token: fields.reset_token,
            login_id: fields.login_id,
            password: fields.password,
        });
        set_metadata(&mut request, self.request_id, nonce, token)?;

        let response = client.reset(request).await.map_err(ServiceError::from)?;
        let response: Option<ResetPasswordResponse> = response.into_inner().into();
        response.ok_or(ServiceError::InfraError("failed to decode response".into()))
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use crate::auth::password::reset::{
        _api::reset::infra::{ResetPasswordResponse, ResetPasswordService},
        _common::reset::infra::ResetPasswordFieldsExtract,
    };

    use crate::auth::{
        _api::service::data::ServiceError,
        auth_ticket::_common::{
            encode::data::EncodeAuthTicketResponse,
            kernel::data::{AuthNonceValue, AuthTokenEncoded, AuthTokenExtract, AuthTokenValue},
        },
        auth_user::_common::kernel::data::AuthUser,
    };

    pub struct StaticResetPasswordService {
        pub user: AuthUser,
    }

    #[async_trait::async_trait]
    impl ResetPasswordService for StaticResetPasswordService {
        async fn reset(
            &self,
            _nonce: AuthNonceValue,
            _token: AuthTokenValue,
            _fields: ResetPasswordFieldsExtract,
        ) -> Result<ResetPasswordResponse, ServiceError> {
            Ok(ResetPasswordResponse::Success(
                EncodeAuthTicketResponse::new(
                    self.user.clone(),
                    AuthTokenEncoded {
                        ticket_token: AuthTokenExtract {
                            token: "TICKET-TOKEN".into(),
                            expires: 0,
                        },
                        api_token: AuthTokenExtract {
                            token: "API-TOKEN".into(),
                            expires: 0,
                        },
                        cloudfront_tokens: HashMap::new(),
                    },
                ),
            ))
        }
    }
}
