use tonic::metadata::MetadataValue;
use tonic::Request;

use crate::auth::auth_ticket::_common::y_protobuf::service::{
    logout_pb_client::LogoutPbClient, LogoutRequestPb,
};

use crate::{
    auth::{
        _api::x_outside_feature::feature::AuthOutsideService,
        auth_ticket::_common::kernel::x_tonic::metadata::{METADATA_NONCE, METADATA_TICKET_TOKEN},
    },
    x_outside_feature::_common::metadata::METADATA_REQUEST_ID,
};

use crate::auth::_api::service::helper::infra_error;

use crate::auth::auth_ticket::_api::logout::infra::LogoutService;

use crate::auth::{
    _api::service::data::ServiceError,
    auth_ticket::_api::kernel::data::{AuthNonceValue, AuthTokenValue},
};

pub struct TonicLogoutService<'a> {
    auth_service_url: &'static str,
    request_id: &'a str,
}

impl<'a> TonicLogoutService<'a> {
    pub const fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            auth_service_url: service.auth_service_url,
            request_id,
        }
    }
}

#[async_trait::async_trait]
impl<'a> LogoutService for TonicLogoutService<'a> {
    async fn logout(
        &self,
        nonce: AuthNonceValue,
        token: AuthTokenValue,
    ) -> Result<(), ServiceError> {
        let mut client = LogoutPbClient::connect(self.auth_service_url)
            .await
            .map_err(infra_error)?;

        let mut request = Request::new(LogoutRequestPb {});
        request.metadata_mut().append(
            METADATA_REQUEST_ID,
            MetadataValue::from_str(self.request_id).map_err(infra_error)?,
        );
        request.metadata_mut().append(
            METADATA_NONCE,
            MetadataValue::from_str(&nonce.extract()).map_err(infra_error)?,
        );
        request.metadata_mut().append(
            METADATA_TICKET_TOKEN,
            MetadataValue::from_str(&token.extract()).map_err(infra_error)?,
        );

        client.logout(request).await.map_err(Into::into).map(|_| ())
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::logout::infra::LogoutService;

    use crate::auth::{
        _api::service::data::ServiceError,
        auth_ticket::_api::kernel::data::{AuthNonceValue, AuthTokenValue},
    };

    pub struct StaticLogoutService;

    #[async_trait::async_trait]
    impl LogoutService for StaticLogoutService {
        async fn logout(
            &self,
            _nonce: AuthNonceValue,
            _token: AuthTokenValue,
        ) -> Result<(), ServiceError> {
            Ok(())
        }
    }
}
