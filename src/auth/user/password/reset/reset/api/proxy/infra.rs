use crate::common::proxy::infra::ProxyCall;

use crate::auth::{
    proxy::data::AuthProxyCallError, ticket::authenticate::proxy::data::ProxyResponseAuthenticated,
};

pub trait ResetPasswordProxyInfra {
    type ProxyCall: ProxyCall<
        Request = String,
        Response = ProxyResponseAuthenticated,
        Error = AuthProxyCallError,
    >;

    fn proxy_call(&self) -> &Self::ProxyCall;
}

pub trait ResetPasswordProxyLogger: Send + Sync {
    fn try_to_reset_password(&self);
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError;
    fn succeed_to_reset_password(
        &self,
        auth: ProxyResponseAuthenticated,
    ) -> ProxyResponseAuthenticated;
}
