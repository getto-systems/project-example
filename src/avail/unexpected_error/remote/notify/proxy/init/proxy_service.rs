use tonic::Request;

use crate::avail::unexpected_error::remote::y_protobuf::service::{
    notify_pb_client::NotifyPbClient, NotifyRequestPb,
};

use crate::example::remote::x_outside_feature::feature::ExampleOutsideService;

use crate::z_lib::remote::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{example::remote::helper::infra_error, z_lib::remote::service::helper::new_endpoint};

use crate::auth::remote::method::set_metadata;

use crate::{
    auth::remote::infra::AuthMetadataContent,
    avail::unexpected_error::remote::notify::infra::NotifyUnexpectedErrorFieldsExtract,
    example::remote::proxy::ExampleProxyService,
};

use crate::example::remote::data::ExampleServiceError;

pub struct ProxyService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer,
}

impl<'a> ProxyService<'a> {
    pub fn new(service: &'a ExampleOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(service.service_url),
        }
    }
}

#[async_trait::async_trait]
impl<'a> ExampleProxyService<NotifyUnexpectedErrorFieldsExtract, ()> for ProxyService<'a> {
    fn name(&self) -> &str {
        "avail.unexpected_error.notify"
    }
    async fn call(
        &self,
        metadata: AuthMetadataContent,
        params: NotifyUnexpectedErrorFieldsExtract,
    ) -> Result<(), ExampleServiceError> {
        let mut client = NotifyPbClient::new(
            new_endpoint(self.service_url)
                .map_err(infra_error)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let mut request = Request::new(NotifyRequestPb { err: params.err });
        set_metadata(&mut request, self.request_id, &self.authorizer, metadata)
            .await
            .map_err(infra_error)?;

        client
            .notify(request)
            .await
            .map_err(ExampleServiceError::from)?;
        Ok(())
    }
}
