use crate::common::proxy::infra::ProxyCall;

use crate::{auth::proxy::data::AuthProxyCallError, common::proxy::data::ProxyResponseBody};

pub trait RequestResetPasswordTokenProxyInfra {
    type ProxyCall: ProxyCall<
        Request = String,
        Response = ProxyResponseBody,
        Error = AuthProxyCallError,
    >;

    fn proxy_call(&self) -> &Self::ProxyCall;
}

pub trait RequestResetPasswordTokenProxyLogger: Send + Sync {
    fn try_to_request_reset_token(&self);
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError;
    fn succeed_to_request_reset_token(&self, response: ProxyResponseBody) -> ProxyResponseBody;
}
