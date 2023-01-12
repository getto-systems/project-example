use crate::auth::x_outside_feature::feature::AuthOutsideDecodingKey;

use crate::auth::init::ActiveAuthorizeWithTokenInfra;

use crate::common::proxy::{
    action::CoreProxyMaterial,
    data::{CoreProxyError, ProxyResponseBody},
    infra::ProxyCall,
};

pub struct ActiveCoreProxyMaterial<'a, P> {
    authorize_with_token: ActiveAuthorizeWithTokenInfra<'a>,
    proxy_call: P,
}

impl<'a, P> ActiveCoreProxyMaterial<'a, P> {
    pub fn new(decoding_key: &'a AuthOutsideDecodingKey, proxy_call: P) -> Self {
        Self {
            authorize_with_token: ActiveAuthorizeWithTokenInfra::new(decoding_key),
            proxy_call,
        }
    }
}

impl<'a, P: ProxyCall<Response = ProxyResponseBody, Error = CoreProxyError>> CoreProxyMaterial
    for ActiveCoreProxyMaterial<'a, P>
{
    type AuthorizeWithToken = ActiveAuthorizeWithTokenInfra<'a>;
    type ProxyCall = P;

    fn authorize_with_token(&self) -> &Self::AuthorizeWithToken {
        &self.authorize_with_token
    }
    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::init::test::StaticAuthorizeWithTokenInfra;

    use crate::common::proxy::{
        action::CoreProxyMaterial,
        data::{CoreProxyError, ProxyResponseBody},
        infra::ProxyCall,
    };

    pub struct StaticCoreProxyMaterial<P> {
        pub authorize_with_token: StaticAuthorizeWithTokenInfra,
        pub proxy_call: P,
    }

    impl<P: ProxyCall<Response = ProxyResponseBody, Error = CoreProxyError>> CoreProxyMaterial
        for StaticCoreProxyMaterial<P>
    {
        type AuthorizeWithToken = StaticAuthorizeWithTokenInfra;
        type ProxyCall = P;

        fn authorize_with_token(&self) -> &Self::AuthorizeWithToken {
            &self.authorize_with_token
        }
        fn proxy_call(&self) -> &Self::ProxyCall {
            &self.proxy_call
        }
    }
}
