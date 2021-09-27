use tonic::Request;

use crate::auth::user::password::_common::y_protobuf::service::{
    change_password_pb_client::ChangePasswordPbClient, ChangePasswordRequestPb,
};

use crate::auth::_common::x_outside_feature::feature::AuthOutsideService;

use crate::z_details::_common::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{
    auth::remote::service::helper::{infra_error, set_metadata},
    z_details::_common::service::helper::new_endpoint,
};

use crate::{
    auth::{
        _api::proxy::AuthProxyService,
        ticket::remote::kernel::infra::AuthMetadataContent,
        user::password::remote::proxy_change::infra::{
            ChangePasswordFieldsExtract, ChangePasswordProxyResponse,
        },
    },
    z_details::_common::service::infra::ServiceAuthorizer,
};

use crate::auth::remote::service::data::AuthServiceError;

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
            new_endpoint(self.service_url)
                .map_err(infra_error)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let mut request = Request::new(ChangePasswordRequestPb {
            current_password: params.current_password,
            new_password: params.new_password,
        });
        set_metadata(
            &mut request,
            self.request_id,
            self.authorizer.fetch_token().await.map_err(infra_error)?,
            metadata,
        )
        .map_err(infra_error)?;

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
