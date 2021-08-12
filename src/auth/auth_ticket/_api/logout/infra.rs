use crate::auth::auth_ticket::_api::kernel::infra::AuthHeaderInfra;

use crate::auth::{
    _api::service::data::ServiceError,
    auth_ticket::_common::kernel::data::{AuthNonceValue, AuthTokenValue},
};

pub trait LogoutInfra {
    type HeaderInfra: AuthHeaderInfra;
    type LogoutService: LogoutService;

    fn header_infra(&self) -> &Self::HeaderInfra;
    fn logout_service(&self) -> &Self::LogoutService;
}

#[async_trait::async_trait]
pub trait LogoutService {
    async fn logout(
        &self,
        nonce: AuthNonceValue,
        token: AuthTokenValue,
    ) -> Result<(), ServiceError>;
}
