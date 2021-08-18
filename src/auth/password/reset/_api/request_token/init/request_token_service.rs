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
    _api::service::data::AuthServiceError, auth_ticket::_common::kernel::data::AuthNonce,
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
        nonce: Option<AuthNonce>,
        fields: RequestResetTokenFieldsExtract,
    ) -> Result<RequestResetTokenResponse, AuthServiceError> {
        let mut client = RequestResetTokenPbClient::connect(self.auth_service_url)
            .await
            .map_err(infra_error)?;

        let mut request = Request::new(RequestResetTokenRequestPb {
            login_id: fields.login_id,
        });
        set_metadata(&mut request, self.request_id, nonce, None)?;

        let response = client
            .request_token(request)
            .await
            .map_err(AuthServiceError::from)?;
        let response: Option<RequestResetTokenResponse> = response.into_inner().into();
        response.ok_or(AuthServiceError::InfraError(
            "failed to decode response".into(),
        ))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::reset::{
        _api::request_token::infra::{RequestResetTokenResponse, RequestResetTokenService},
        _common::request_token::infra::RequestResetTokenFieldsExtract,
    };

    use crate::auth::{
        _api::service::data::AuthServiceError, auth_ticket::_common::kernel::data::AuthNonce,
    };

    pub struct StaticRequestResetTokenService;

    #[async_trait::async_trait]
    impl RequestResetTokenService for StaticRequestResetTokenService {
        async fn request_token(
            &self,
            _nonce: Option<AuthNonce>,
            _fields: RequestResetTokenFieldsExtract,
        ) -> Result<RequestResetTokenResponse, AuthServiceError> {
            Ok(RequestResetTokenResponse::Success)
        }
    }
}
