use tonic::Request;

use crate::auth::ticket::y_protobuf::service::{
    validate_api_token_pb_client::ValidateApiTokenPbClient, ValidateApiTokenRequestPb,
};

use crate::auth::x_outside_feature::remote::common::feature::AuthOutsideService;

use crate::z_lib::remote::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{auth::proxy::helper::infra_error, z_lib::remote::service::helper::new_endpoint};

use crate::auth::proxy::method::set_metadata;

use crate::auth::ticket::validate::infra::{AuthMetadataContent, ValidateService};

use crate::auth::{proxy::data::AuthProxyError, user::kernel::data::RequireAuthRoles};

pub struct TonicValidateService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer,
}

impl<'a> TonicValidateService<'a> {
    pub fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(service.service_url),
        }
    }
}

#[async_trait::async_trait]
impl<'a> ValidateService for TonicValidateService<'a> {
    async fn validate(
        &self,
        metadata: AuthMetadataContent,
        require_roles: RequireAuthRoles,
    ) -> Result<(), AuthProxyError> {
        validate(self, metadata, require_roles).await
    }
}

async fn validate<'a>(
    service: &TonicValidateService<'a>,
    metadata: AuthMetadataContent,
    require_roles: RequireAuthRoles,
) -> Result<(), AuthProxyError> {
    let mut client = ValidateApiTokenPbClient::new(
        new_endpoint(service.service_url)
            .map_err(infra_error)?
            .connect()
            .await
            .map_err(infra_error)?,
    );

    let request: ValidateApiTokenRequestPb = require_roles.into();
    let mut request = Request::new(request);
    set_metadata(
        &mut request,
        service.request_id,
        &service.authorizer,
        metadata,
    )
    .await
    .map_err(infra_error)?;

    client
        .validate(request)
        .await
        .map_err(AuthProxyError::from)?;

    Ok(())
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::validate::infra::{AuthMetadataContent, ValidateService};

    use crate::auth::{proxy::data::AuthProxyError, user::kernel::data::RequireAuthRoles};

    pub struct StaticValidateService;

    #[async_trait::async_trait]
    impl ValidateService for StaticValidateService {
        async fn validate(
            &self,
            _metadata: AuthMetadataContent,
            _require_roles: RequireAuthRoles,
        ) -> Result<(), AuthProxyError> {
            Ok(())
        }
    }
}
