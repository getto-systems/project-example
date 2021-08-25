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
    _api::service::data::AuthServiceError,
    auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
};

pub struct TonicResetPasswordService<'a> {
    service_url: &'static str,
    request_id: &'a str,
}

impl<'a> TonicResetPasswordService<'a> {
    pub const fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
        }
    }
}

#[async_trait::async_trait]
impl<'a> ResetPasswordService for TonicResetPasswordService<'a> {
    async fn reset(
        &self,
        nonce: Option<AuthNonce>,
        token: Option<AuthToken>,
        fields: ResetPasswordFieldsExtract,
    ) -> Result<ResetPasswordResponse, AuthServiceError> {
        let mut client = ResetPasswordPbClient::connect(self.service_url)
            .await
            .map_err(infra_error)?;

        let mut request = Request::new(ResetPasswordRequestPb {
            reset_token: fields.reset_token,
            login_id: fields.login_id,
            password: fields.password,
        });
        set_metadata(&mut request, self.request_id, nonce, token)?;

        let response = client.reset(request).await.map_err(AuthServiceError::from)?;
        let response: Option<ResetPasswordResponse> = response.into_inner().into();
        response.ok_or(AuthServiceError::InfraError("failed to decode response".into()))
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
        _api::service::data::AuthServiceError,
        auth_ticket::_common::{
            encode::data::AuthTicketEncoded,
            kernel::data::{AuthNonce, AuthTokenEncoded, AuthTokenExtract, AuthToken},
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
            _nonce: Option<AuthNonce>,
            _token: Option<AuthToken>,
            _fields: ResetPasswordFieldsExtract,
        ) -> Result<ResetPasswordResponse, AuthServiceError> {
            Ok(ResetPasswordResponse::Success(AuthTicketEncoded {
                user: self.user.clone().extract(),
                token: AuthTokenEncoded {
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
            }))
        }
    }
}
