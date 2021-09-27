use tonic::Request;

use crate::auth::ticket::_common::y_protobuf::service::{
    renew_auth_ticket_pb_client::RenewAuthTicketPbClient, RenewAuthTicketRequestPb,
};

use crate::auth::remote::x_outside_feature::common::feature::AuthOutsideService;

use crate::z_details::_common::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{
    auth::remote::service::helper::{infra_error, set_metadata},
    z_details::_common::service::helper::new_endpoint,
};

use crate::{
    auth::{
        remote::service::proxy::AuthProxyService,
        ticket::remote::kernel::infra::AuthMetadataContent,
    },
    z_details::_common::service::infra::ServiceAuthorizer,
};

use crate::auth::{
    remote::service::data::AuthServiceError, ticket::remote::encode::data::AuthTicketEncoded,
};

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
impl<'a> AuthProxyService<(), AuthTicketEncoded> for ProxyService<'a> {
    fn name(&self) -> &str {
        "auth.auth_ticket.renew"
    }
    async fn call(
        &self,
        metadata: AuthMetadataContent,
        _params: (),
    ) -> Result<AuthTicketEncoded, AuthServiceError> {
        let mut client = RenewAuthTicketPbClient::new(
            new_endpoint(self.service_url)
                .map_err(infra_error)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let mut request = Request::new(RenewAuthTicketRequestPb {});
        set_metadata(
            &mut request,
            self.request_id,
            self.authorizer.fetch_token().await.map_err(infra_error)?,
            metadata,
        )
        .map_err(infra_error)?;

        let response = client
            .renew(request)
            .await
            .map_err(AuthServiceError::from)?;

        let ticket: Option<AuthTicketEncoded> = response.into_inner().into();
        ticket.ok_or(AuthServiceError::InfraError(
            "failed to decode response".into(),
        ))
    }
}
