mod proxy_call;

use proxy_call::TonicResetPasswordProxyCall;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::auth::user::password::reset::reset::proxy::action::{
    ResetPasswordProxyAction, ResetPasswordProxyMaterial,
};

pub struct ActiveResetPasswordProxyMaterial<'a> {
    proxy_call: TonicResetPasswordProxyCall<'a>,
}

impl<'a> ActiveResetPasswordProxyMaterial<'a> {
    pub fn action(
        feature: &'a ProxyAppFeature,
        request_id: RequestId,
    ) -> ResetPasswordProxyAction<Self> {
        ResetPasswordProxyAction::with_material(Self {
            proxy_call: TonicResetPasswordProxyCall::new(&feature.auth, request_id),
        })
    }
}

#[async_trait::async_trait]
impl<'a> ResetPasswordProxyMaterial for ActiveResetPasswordProxyMaterial<'a> {
    type ProxyCall = TonicResetPasswordProxyCall<'a>;

    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

#[cfg(test)]
pub mod test {
    pub use super::proxy_call::test::*;

    use crate::auth::user::password::reset::reset::proxy::action::ResetPasswordProxyMaterial;

    pub struct StaticResetPasswordProxyMaterial;

    #[async_trait::async_trait]
    impl ResetPasswordProxyMaterial for StaticResetPasswordProxyMaterial {
        type ProxyCall = StaticResetPasswordProxyCall;

        fn proxy_call(&self) -> &Self::ProxyCall {
            &StaticResetPasswordProxyCall
        }
    }
}
