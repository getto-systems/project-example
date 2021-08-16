use tonic::Request;

use crate::auth::password::_common::y_protobuf::service::{
    authenticate_password_pb_client::AuthenticatePasswordPbClient, AuthenticatePasswordRequestPb,
};

use crate::auth::_api::x_outside_feature::feature::AuthOutsideService;

use crate::auth::_api::service::helper::{infra_error, set_metadata};

use crate::auth::password::{
    _api::authenticate::infra::{AuthenticatePasswordResponse, AuthenticatePasswordService},
    _common::authenticate::infra::AuthenticatePasswordFieldsExtract,
};

use crate::auth::{
    _api::service::data::ServiceError,
    auth_ticket::_common::kernel::data::{AuthNonceValue, AuthTokenValue},
};

pub struct TonicAuthenticatePasswordService<'a> {
    auth_service_url: &'static str,
    request_id: &'a str,
}

impl<'a> TonicAuthenticatePasswordService<'a> {
    pub const fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            auth_service_url: service.auth_service_url,
            request_id,
        }
    }
}

#[async_trait::async_trait]
impl<'a> AuthenticatePasswordService for TonicAuthenticatePasswordService<'a> {
    async fn authenticate(
        &self,
        nonce: Option<AuthNonceValue>,
        token: Option<AuthTokenValue>,
        fields: AuthenticatePasswordFieldsExtract,
    ) -> Result<AuthenticatePasswordResponse, ServiceError> {
        let mut client = AuthenticatePasswordPbClient::connect(self.auth_service_url)
            .await
            .map_err(infra_error)?;

        let mut request = Request::new(AuthenticatePasswordRequestPb {
            login_id: fields.login_id,
            password: fields.password,
        });
        set_metadata(&mut request, self.request_id, nonce, token)?;

        let response = client
            .authenticate(request)
            .await
            .map_err(ServiceError::from)?;
        let response: Option<AuthenticatePasswordResponse> = response.into_inner().into();
        response.ok_or(ServiceError::InfraError("failed to decode response".into()))
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use crate::auth::password::{
        _api::authenticate::infra::{AuthenticatePasswordResponse, AuthenticatePasswordService},
        _common::authenticate::infra::AuthenticatePasswordFieldsExtract,
    };

    use crate::auth::{
        _api::service::data::ServiceError,
        auth_ticket::_common::{
            encode::data::AuthTicketEncoded,
            kernel::data::{AuthNonceValue, AuthTokenEncoded, AuthTokenExtract, AuthTokenValue},
        },
        auth_user::_common::kernel::data::AuthUser,
    };

    pub struct StaticAuthenticatePasswordService {
        pub user: AuthUser,
    }

    #[async_trait::async_trait]
    impl AuthenticatePasswordService for StaticAuthenticatePasswordService {
        async fn authenticate(
            &self,
            _nonce: Option<AuthNonceValue>,
            _token: Option<AuthTokenValue>,
            _fields: AuthenticatePasswordFieldsExtract,
        ) -> Result<AuthenticatePasswordResponse, ServiceError> {
            Ok(AuthenticatePasswordResponse::Success(AuthTicketEncoded {
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
