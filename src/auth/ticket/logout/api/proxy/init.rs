mod proxy_call;

use proxy_call::TonicLogoutProxyCall;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::auth::ticket::authenticate::init::ActiveAuthenticateWithTokenInfra;

use crate::auth::ticket::logout::proxy::action::{LogoutProxyAction, LogoutProxyMaterial};

pub struct ActiveLogoutProxyMaterial<'a> {
    authenticate_with_token: ActiveAuthenticateWithTokenInfra<'a>,
    proxy_call: TonicLogoutProxyCall<'a>,
}

impl<'a> ActiveLogoutProxyMaterial<'a> {
    pub fn action(
        feature: &'a ProxyAppFeature,
        request_id: RequestId,
    ) -> LogoutProxyAction<Self> {
        LogoutProxyAction::with_material(Self {
            authenticate_with_token: ActiveAuthenticateWithTokenInfra::new(
                &feature.auth.decoding_key,
            ),
            proxy_call: TonicLogoutProxyCall::new(&feature.auth, request_id),
        })
    }
}

#[async_trait::async_trait]
impl<'a> LogoutProxyMaterial for ActiveLogoutProxyMaterial<'a> {
    type AuthenticateWithToken = ActiveAuthenticateWithTokenInfra<'a>;
    type ProxyCall = TonicLogoutProxyCall<'a>;

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

    use crate::auth::ticket::authenticate::init::test::StaticAuthenticateWithTokenInfra;

    use crate::auth::ticket::logout::proxy::action::LogoutProxyMaterial;

    pub struct StaticLogoutProxyMaterial {
        pub authenticate_with_token: StaticAuthenticateWithTokenInfra,
    }

    #[async_trait::async_trait]
    impl LogoutProxyMaterial for StaticLogoutProxyMaterial {
        type AuthenticateWithToken = StaticAuthenticateWithTokenInfra;
        type ProxyCall = StaticLogoutProxyCall;

        fn authenticate_with_token(&self) -> &Self::AuthenticateWithToken {
            &self.authenticate_with_token
        }
        fn proxy_call(&self) -> &Self::ProxyCall {
            &StaticLogoutProxyCall
        }
    }
}
