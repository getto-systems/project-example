use crate::{
    auth::{
        proxy::data::AuthProxyCallError,
        ticket::{
            authenticate::{
                data::CheckAuthenticateTokenSuccess, proxy::data::ProxyResponseAuthenticated,
            },
            kernel::data::ValidateAuthenticateTokenError,
        },
    },
    common::proxy::infra::ProxyCall,
};

pub trait AuthenticateWithTokenProxyInfra {
    type ProxyCall: ProxyCall<
        Request = CheckAuthenticateTokenSuccess,
        Response = ProxyResponseAuthenticated,
        Error = AuthProxyCallError,
    >;

    fn proxy_call(&self) -> &Self::ProxyCall;
}

pub trait AuthenticateWithTokenProxyLogger: Send + Sync {
    fn try_to_authenticate_with_token(&self);
    fn invalid_request(
        &self,
        err: ValidateAuthenticateTokenError,
    ) -> ValidateAuthenticateTokenError;
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError;
    fn authenticated(&self, auth: ProxyResponseAuthenticated) -> ProxyResponseAuthenticated;
}
