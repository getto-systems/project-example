use tonic::Request;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideService;

use crate::auth::password::reset::_common::y_protobuf::service::{
    request_reset_token_pb_client::RequestResetTokenPbClient, RequestResetTokenRequestPb,
};

use crate::auth::_api::service::helper::{infra_error, set_metadata};

use crate::auth::password::reset::{
    _api::request_token::infra::{RequestResetTokenResponse, RequestResetTokenService},
    _common::request_token::infra::RequestResetTokenFieldsExtract,
};

use crate::auth::{
    _api::service::data::ServiceError,
    auth_ticket::_common::kernel::data::{AuthNonceValue, AuthTokenValue},
};

pub struct TonicRequestResetTokenService<'a> {
    auth_service_url: &'static str,
    request_id: &'a str,
}

impl<'a> TonicRequestResetTokenService<'a> {
    pub const fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            auth_service_url: service.auth_service_url,
            request_id,
        }
    }
}

#[async_trait::async_trait]
impl<'a> RequestResetTokenService for TonicRequestResetTokenService<'a> {
    async fn request_token(
        &self,
        nonce: AuthNonceValue,
        token: AuthTokenValue,
        fields: RequestResetTokenFieldsExtract,
    ) -> Result<RequestResetTokenResponse, ServiceError> {
        let mut client = RequestResetTokenPbClient::connect(self.auth_service_url)
            .await
            .map_err(infra_error)?;

        let mut request = Request::new(RequestResetTokenRequestPb {
            login_id: fields.login_id,
        });
        set_metadata(&mut request, self.request_id, nonce, token)?;

        let response = client
            .request_token(request)
            .await
            .map_err(ServiceError::from)?;
        let response: Option<RequestResetTokenResponse> = response.into_inner().into();
        response.ok_or(ServiceError::InfraError("failed to decode response".into()))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::reset::{
        _api::request_token::infra::{RequestResetTokenResponse, RequestResetTokenService},
        _common::request_token::infra::RequestResetTokenFieldsExtract,
    };

    use crate::auth::{
        _api::service::data::ServiceError,
        auth_ticket::_common::kernel::data::{AuthNonceValue, AuthTokenValue},
    };

    pub struct StaticRequestResetTokenService;

    #[async_trait::async_trait]
    impl RequestResetTokenService for StaticRequestResetTokenService {
        async fn request_token(
            &self,
            _nonce: AuthNonceValue,
            _token: AuthTokenValue,
            _fields: RequestResetTokenFieldsExtract,
        ) -> Result<RequestResetTokenResponse, ServiceError> {
            Ok(RequestResetTokenResponse::Success)
        }
    }
}
