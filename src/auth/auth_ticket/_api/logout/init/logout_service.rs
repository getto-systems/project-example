use tonic::Request;

use crate::auth::auth_ticket::_common::y_protobuf::service::{
    logout_pb_client::LogoutPbClient, LogoutRequestPb,
};

use crate::auth::_api::x_outside_feature::feature::AuthOutsideService;

use crate::auth::_api::service::helper::{infra_error, set_metadata};

use crate::auth::auth_ticket::_api::logout::infra::LogoutService;

use crate::auth::{
    _api::service::data::ServiceError,
    auth_ticket::_common::kernel::data::{AuthNonceValue, AuthTokenValue},
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
        nonce: Option<AuthNonceValue>,
        token: Option<AuthTokenValue>,
    ) -> Result<(), ServiceError> {
        let mut client = LogoutPbClient::connect(self.auth_service_url)
            .await
            .map_err(infra_error)?;

        let mut request = Request::new(LogoutRequestPb {});
        set_metadata(&mut request, self.request_id, nonce, token)?;

        client.logout(request).await.map_err(ServiceError::from)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::logout::infra::LogoutService;

    use crate::auth::{
        _api::service::data::ServiceError,
        auth_ticket::_common::kernel::data::{AuthNonceValue, AuthTokenValue},
    };

    pub struct StaticLogoutService;

    #[async_trait::async_trait]
    impl LogoutService for StaticLogoutService {
        async fn logout(
            &self,
            _nonce: Option<AuthNonceValue>,
            _token: Option<AuthTokenValue>,
        ) -> Result<(), ServiceError> {
            Ok(())
        }
    }
}
