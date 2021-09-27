use tonic::Request;

use crate::z_details::_common::service::init::authorizer::GoogleServiceAuthorizer;

use crate::auth::password::_common::y_protobuf::service::{
    change_password_pb_client::ChangePasswordPbClient, ChangePasswordRequestPb,
};

use crate::auth::_common::x_outside_feature::feature::AuthOutsideService;

use crate::auth::_common::service::helper::{
    infra_error, new_endpoint, set_authorization, set_metadata,
};

use crate::auth::{
    _api::proxy::AuthProxyService,
    ticket::remote::kernel::infra::AuthMetadataContent,
    password::remote::proxy_change::infra::{
        ChangePasswordFieldsExtract, ChangePasswordProxyResponse,
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
impl<'a> AuthProxyService<ChangePasswordFieldsExtract, ChangePasswordProxyResponse>
    for ProxyService<'a>
{
    fn name(&self) -> &str {
        "auth.password.change"
    }
    async fn call(
        &self,
        metadata: AuthMetadataContent,
        params: ChangePasswordFieldsExtract,
    ) -> Result<ChangePasswordProxyResponse, AuthServiceError> {
        let mut client = ChangePasswordPbClient::new(
            new_endpoint(self.service_url)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let mut request = Request::new(ChangePasswordRequestPb {
            current_password: params.current_password,
            new_password: params.new_password,
        });
        set_authorization(&mut request, &self.authorizer).await?;
        set_metadata(&mut request, self.request_id, metadata)?;

        let response = client
            .change(request)
            .await
            .map_err(AuthServiceError::from)?;
        let response: Option<ChangePasswordProxyResponse> = response.into_inner().into();
        response.ok_or(AuthServiceError::InfraError(
            "failed to decode response".into(),
        ))
    }
}
