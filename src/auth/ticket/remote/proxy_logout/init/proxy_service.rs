use tonic::Request;

use crate::auth::ticket::_common::y_protobuf::service::{
    logout_pb_client::LogoutPbClient, LogoutRequestPb,
};

use crate::auth::remote::x_outside_feature::common::feature::AuthOutsideService;

use crate::z_lib::remote::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{
    auth::remote::service::helper::{infra_error, set_metadata},
    z_lib::remote::service::helper::new_endpoint,
};

use crate::{
    auth::remote::{infra::AuthMetadataContent, service::proxy::AuthProxyService},
    z_lib::remote::service::infra::ServiceAuthorizer,
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
impl<'a> AuthProxyService<(), ()> for ProxyService<'a> {
    fn name(&self) -> &str {
        "auth.auth_ticket.logout"
    }
    async fn call(
        &self,
        metadata: AuthMetadataContent,
        _params: (),
    ) -> Result<(), AuthServiceError> {
        let mut client = LogoutPbClient::new(
            new_endpoint(self.service_url)
                .map_err(infra_error)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let mut request = Request::new(LogoutRequestPb {});
        set_metadata(
            &mut request,
            self.request_id,
            self.authorizer.fetch_token().await.map_err(infra_error)?,
            metadata,
        )
        .map_err(infra_error)?;

        client
            .logout(request)
            .await
            .map_err(AuthServiceError::from)?;
        Ok(())
    }
}
