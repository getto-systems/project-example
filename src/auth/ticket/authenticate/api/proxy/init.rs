mod proxy_call;

use proxy_call::TonicAuthenticateWithTokenProxyCall;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::auth::ticket::authenticate::init::ActiveAuthenticateWithTokenInfra;

use crate::auth::ticket::authenticate::proxy::action::{
    AuthenticateWithTokenProxyAction, AuthenticateWithTokenProxyMaterial,
};

pub struct ActiveAuthenticateWithTokenProxyMaterial<'a> {
    authenticate_with_token: ActiveAuthenticateWithTokenInfra<'a>,
    proxy_call: TonicAuthenticateWithTokenProxyCall<'a>,
}

impl<'a> ActiveAuthenticateWithTokenProxyMaterial<'a> {
    pub fn action(
        feature: &'a ProxyAppFeature,
        request_id: RequestId,
    ) -> AuthenticateWithTokenProxyAction<Self> {
        AuthenticateWithTokenProxyAction::with_material(Self {
            authenticate_with_token: ActiveAuthenticateWithTokenInfra::new(
                &feature.auth.decoding_key,
            ),
            proxy_call: TonicAuthenticateWithTokenProxyCall::new(&feature.auth, request_id),
        })
    }
}

#[async_trait::async_trait]
impl<'a> AuthenticateWithTokenProxyMaterial for ActiveAuthenticateWithTokenProxyMaterial<'a> {
    type AuthenticateWithToken = ActiveAuthenticateWithTokenInfra<'a>;
    type ProxyCall = TonicAuthenticateWithTokenProxyCall<'a>;

    fn authenticate_with_token(&self) -> &Self::AuthenticateWithToken {
        &self.authenticate_with_token
    }
    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

#[cfg(test)]
pub mod test {
    pub use super::proxy_call::test::*;

    use crate::auth::ticket::authenticate::{
        init::test::StaticAuthenticateWithTokenInfra,
        proxy::action::AuthenticateWithTokenProxyMaterial,
    };

    pub struct StaticAuthenticateWithTokenProxyMaterial {
        pub authenticate_with_token: StaticAuthenticateWithTokenInfra,
    }

    #[async_trait::async_trait]
    impl AuthenticateWithTokenProxyMaterial for StaticAuthenticateWithTokenProxyMaterial {
        type AuthenticateWithToken = StaticAuthenticateWithTokenInfra;
        type ProxyCall = StaticAuthenticateWithTokenProxyCall;

        fn authenticate_with_token(&self) -> &Self::AuthenticateWithToken {
            &self.authenticate_with_token
        }
        fn proxy_call(&self) -> &Self::ProxyCall {
            &StaticAuthenticateWithTokenProxyCall
        }
    }
}
