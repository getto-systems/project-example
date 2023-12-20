use crate::common::proxy::infra::ProxyCall;

use crate::auth::{
    proxy::data::AuthProxyCallError, ticket::authenticate::proxy::data::ProxyResponseAuthenticated,
};

pub trait AuthenticateWithPasswordProxyInfra {
    type ProxyCall: ProxyCall<
        Request = String,
        Response = ProxyResponseAuthenticated,
        Error = AuthProxyCallError,
    >;

    fn proxy_call(&self) -> &Self::ProxyCall;
}

pub trait AuthenticateWithPasswordProxyLogger: Send + Sync {
    fn try_to_authenticate_with_password(&self);
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError;
    fn authenticated(&self, auth: ProxyResponseAuthenticated) -> ProxyResponseAuthenticated;
}
