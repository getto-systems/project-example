use crate::common::proxy::infra::ProxyCall;

use crate::{
    auth::{
        proxy::data::AuthProxyCallError,
        ticket::{
            authenticate::data::CheckAuthenticateTokenSuccess,
            kernel::data::ValidateAuthenticateTokenError,
        },
    },
    common::proxy::data::ProxyResponseBody,
};

pub trait LogoutProxyInfra {
    type ProxyCall: ProxyCall<
        Request = CheckAuthenticateTokenSuccess,
        Response = ProxyResponseBody,
        Error = AuthProxyCallError,
    >;

    fn proxy_call(&self) -> &Self::ProxyCall;
}

pub trait LogoutProxyLogger: Send + Sync {
    fn try_to_logout(&self);
    fn invalid_request(
        &self,
        err: ValidateAuthenticateTokenError,
    ) -> ValidateAuthenticateTokenError;
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError;
    fn succeed_to_logout(&self, auth: ProxyResponseBody) -> ProxyResponseBody;
}
