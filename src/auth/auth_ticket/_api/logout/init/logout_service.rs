use tonic::Request;

use crate::auth::auth_ticket::_common::y_protobuf::service::{
    logout_pb_client::LogoutPbClient, LogoutRequestPb,
};

use crate::auth::_api::x_outside_feature::feature::AuthOutsideService;

use crate::z_details::_common::service::init::authorizer::GoogleServiceAuthorizer;

use crate::auth::_common::service::helper::{
    infra_error, new_endpoint, set_authorization, set_metadata,
};

use crate::auth::auth_ticket::{
    _api::logout::infra::LogoutService, _common::kernel::infra::AuthServiceMetadataContent,
};

use crate::auth::_common::service::data::AuthServiceError;

pub struct TonicLogoutService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer,
}

impl<'a> TonicLogoutService<'a> {
    pub fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(service.service_url),
        }
    }
}

#[async_trait::async_trait]
impl<'a> LogoutService for TonicLogoutService<'a> {
    async fn logout(&self, metadata: AuthServiceMetadataContent) -> Result<(), AuthServiceError> {
        let mut client = LogoutPbClient::new(
            new_endpoint(self.service_url)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let mut request = Request::new(LogoutRequestPb {});
        set_authorization(&mut request, &self.authorizer).await?;
        set_metadata(&mut request, self.request_id, metadata)?;

        client
            .logout(request)
            .await
            .map_err(AuthServiceError::from)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::{
        _api::logout::infra::LogoutService, _common::kernel::infra::AuthServiceMetadataContent,
    };

    use crate::auth::_common::service::data::AuthServiceError;

    pub struct StaticLogoutService;

    #[async_trait::async_trait]
    impl LogoutService for StaticLogoutService {
        async fn logout(
            &self,
            _metadata: AuthServiceMetadataContent,
        ) -> Result<(), AuthServiceError> {
            Ok(())
        }
    }
}
