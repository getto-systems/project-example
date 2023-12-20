use crate::{
    auth::ticket::authorize::infra::AuthorizeTokenDecoder, common::proxy::infra::ProxyCall,
};

use crate::auth::{
    proxy::data::AuthProxyCallError,
    ticket::{
        authorize::data::AuthorizeSuccess,
        kernel::data::{
            AuthPermissionError, AuthPermissionRequired, DecodeAuthorizeTokenError,
            ValidateAuthorizeTokenError,
        },
    },
};

pub trait AuthorizeProxyInfra {
    type TokenDecoder: AuthorizeTokenDecoder;
    type ProxyCall: ProxyCall<
        Request = AuthPermissionRequired,
        Response = AuthorizeSuccess,
        Error = AuthProxyCallError,
    >;

    fn token_decoder(&self) -> &Self::TokenDecoder;
    fn proxy_call(&self) -> &Self::ProxyCall;
}

pub trait AuthorizeProxyLogger: Send + Sync {
    fn try_to_authorize(&self);
    fn invalid_request(&self, err: ValidateAuthorizeTokenError) -> ValidateAuthorizeTokenError;
    fn invalid_token(&self, err: DecodeAuthorizeTokenError) -> DecodeAuthorizeTokenError;
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError;
    fn forbidden(&self, err: AuthPermissionError) -> AuthPermissionError;
    fn authorized(&self, auth: AuthorizeSuccess) -> AuthorizeSuccess;
}
