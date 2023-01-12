mod proxy_call;

use proxy_call::TonicAuthenticateWithPasswordProxyCall;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::auth::user::password::authenticate::proxy::action::{
    AuthenticateWithPasswordProxyAction, AuthenticateWithPasswordProxyMaterial,
};

pub struct ActiveAuthenticateWithPasswordProxyMaterial<'a> {
    proxy_call: TonicAuthenticateWithPasswordProxyCall<'a>,
}

impl<'a> ActiveAuthenticateWithPasswordProxyMaterial<'a> {
    pub fn action(
        feature: &'a ProxyAppFeature,
        request_id: RequestId,
    ) -> AuthenticateWithPasswordProxyAction<Self> {
        AuthenticateWithPasswordProxyAction::with_material(Self {
            proxy_call: TonicAuthenticateWithPasswordProxyCall::new(&feature.auth, request_id),
        })
    }
}

#[async_trait::async_trait]
impl<'a> AuthenticateWithPasswordProxyMaterial for ActiveAuthenticateWithPasswordProxyMaterial<'a> {
    type ProxyCall = TonicAuthenticateWithPasswordProxyCall<'a>;

    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

#[cfg(test)]
pub mod test {
    pub use super::proxy_call::test::*;

    use crate::auth::user::password::authenticate::proxy::action::AuthenticateWithPasswordProxyMaterial;

    pub struct StaticAuthenticateWithPasswordProxyMaterial;

    #[async_trait::async_trait]
    impl AuthenticateWithPasswordProxyMaterial for StaticAuthenticateWithPasswordProxyMaterial {
        type ProxyCall = StaticAuthenticateWithPasswordProxyCall;

        fn proxy_call(&self) -> &Self::ProxyCall {
            &StaticAuthenticateWithPasswordProxyCall
        }
    }
}
