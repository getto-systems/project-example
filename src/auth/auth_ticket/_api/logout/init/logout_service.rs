use tonic::Request;

use crate::auth::auth_ticket::_common::y_protobuf::service::{
    logout_pb_client::LogoutPbClient, LogoutRequestPb,
};

use crate::auth::_api::x_outside_feature::feature::AuthOutsideService;

use crate::auth::_api::service::helper::{infra_error, set_metadata};

use crate::auth::auth_ticket::_api::logout::infra::LogoutService;

use crate::auth::{
    _api::service::data::AuthServiceError,
    auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
};

pub struct TonicLogoutService<'a> {
    service_url: &'static str,
    request_id: &'a str,
}

impl<'a> TonicLogoutService<'a> {
    pub const fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
        }
    }
}

#[async_trait::async_trait]
impl<'a> LogoutService for TonicLogoutService<'a> {
    async fn logout(
        &self,
        nonce: Option<AuthNonce>,
        token: Option<AuthToken>,
    ) -> Result<(), AuthServiceError> {
        let mut client = LogoutPbClient::connect(self.service_url)
            .await
            .map_err(infra_error)?;

        let mut request = Request::new(LogoutRequestPb {});
        set_metadata(&mut request, self.request_id, nonce, token)?;

        client.logout(request).await.map_err(AuthServiceError::from)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::logout::infra::LogoutService;

    use crate::auth::{
        _api::service::data::AuthServiceError,
        auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
    };

    pub struct StaticLogoutService;

    #[async_trait::async_trait]
    impl LogoutService for StaticLogoutService {
        async fn logout(
            &self,
            _nonce: Option<AuthNonce>,
            _token: Option<AuthToken>,
        ) -> Result<(), AuthServiceError> {
            Ok(())
        }
    }
}
