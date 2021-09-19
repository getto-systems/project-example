use tonic::Request;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideService;

use crate::auth::password::reset::_common::y_protobuf::service::{
    request_reset_token_pb_client::RequestResetTokenPbClient, RequestResetTokenRequestPb,
};

use crate::z_details::_common::service::init::authorizer::GoogleServiceAuthorizer;

use crate::auth::_common::service::helper::{
    infra_error, new_endpoint, set_authorization, set_metadata,
};

use crate::auth::{
    auth_ticket::_common::kernel::infra::AuthServiceMetadataContent,
    password::reset::{
        _api::request_token::infra::{RequestResetTokenResponse, RequestResetTokenService},
        _common::request_token::infra::RequestResetTokenFieldsExtract,
    },
};

use crate::auth::_common::service::data::AuthServiceError;

pub struct TonicRequestResetTokenService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer,
}

impl<'a> TonicRequestResetTokenService<'a> {
    pub fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(service.service_url),
        }
    }
}

#[async_trait::async_trait]
impl<'a> RequestResetTokenService for TonicRequestResetTokenService<'a> {
    async fn request_token(
        &self,
        metadata: AuthServiceMetadataContent,
        fields: RequestResetTokenFieldsExtract,
    ) -> Result<RequestResetTokenResponse, AuthServiceError> {
        let mut client = RequestResetTokenPbClient::new(
            new_endpoint(self.service_url)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let mut request = Request::new(RequestResetTokenRequestPb {
            login_id: fields.login_id,
        });
        set_authorization(&mut request, &self.authorizer).await?;
        set_metadata(&mut request, self.request_id, metadata)?;

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
    use crate::auth::{
        auth_ticket::_common::kernel::infra::AuthServiceMetadataContent,
        password::reset::{
            _api::request_token::infra::{RequestResetTokenResponse, RequestResetTokenService},
            _common::request_token::infra::RequestResetTokenFieldsExtract,
        },
    };

    use crate::auth::_common::service::data::AuthServiceError;

    pub struct StaticRequestResetTokenService;

    #[async_trait::async_trait]
    impl RequestResetTokenService for StaticRequestResetTokenService {
        async fn request_token(
            &self,
            _metadata: AuthServiceMetadataContent,
            _fields: RequestResetTokenFieldsExtract,
        ) -> Result<RequestResetTokenResponse, AuthServiceError> {
            Ok(RequestResetTokenResponse::Success)
        }
    }
}
