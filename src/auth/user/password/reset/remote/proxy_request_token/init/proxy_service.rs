use tonic::Request;

use crate::z_details::_common::service::init::authorizer::GoogleServiceAuthorizer;

use crate::auth::user::password::reset::_common::y_protobuf::service::{
    request_reset_token_pb_client::RequestResetTokenPbClient, RequestResetTokenRequestPb,
};

use crate::auth::_common::x_outside_feature::feature::AuthOutsideService;

use crate::auth::_common::service::helper::{
    infra_error, new_endpoint, set_authorization, set_metadata,
};

use crate::auth::{
    _api::proxy::AuthProxyService,
    ticket::remote::kernel::infra::AuthMetadataContent,
    user::password::reset::remote::proxy_request_token::infra::{
        RequestResetTokenFieldsExtract, RequestResetTokenProxyResponse,
    },
};

use crate::auth::_common::service::data::AuthServiceError;

pub struct ProxyService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer,
}

impl<'a> ProxyService<'a> {
    pub fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(service.service_url),
        }
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyService<RequestResetTokenFieldsExtract, RequestResetTokenProxyResponse>
    for ProxyService<'a>
{
    fn name(&self) -> &str {
        "auth.password.reset.request_token"
    }
    async fn call(
        &self,
        metadata: AuthMetadataContent,
        params: RequestResetTokenFieldsExtract,
    ) -> Result<RequestResetTokenProxyResponse, AuthServiceError> {
        let mut client = RequestResetTokenPbClient::new(
            new_endpoint(self.service_url)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let mut request = Request::new(RequestResetTokenRequestPb {
            login_id: params.login_id,
        });
        set_authorization(&mut request, &self.authorizer).await?;
        set_metadata(&mut request, self.request_id, metadata)?;

        let response = client
            .request_token(request)
            .await
            .map_err(AuthServiceError::from)?;
        let response: Option<RequestResetTokenProxyResponse> = response.into_inner().into();
        response.ok_or(AuthServiceError::InfraError(
            "failed to decode response".into(),
        ))
    }
}
