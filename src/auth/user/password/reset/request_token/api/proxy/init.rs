mod proxy_call;

use proxy_call::TonicRequestResetTokenProxyCall;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::auth::user::password::reset::request_token::proxy::action::{
    RequestResetTokenProxyAction, RequestResetTokenProxyMaterial,
};

pub struct ActiveRequestResetTokenProxyMaterial<'a> {
    proxy_call: TonicRequestResetTokenProxyCall<'a>,
}

impl<'a> ActiveRequestResetTokenProxyMaterial<'a> {
    pub fn action(
        feature: &'a ProxyAppFeature,
        request_id: RequestId,
    ) -> RequestResetTokenProxyAction<Self> {
        RequestResetTokenProxyAction::with_material(Self {
            proxy_call: TonicRequestResetTokenProxyCall::new(&feature.auth, request_id),
        })
    }
}

#[async_trait::async_trait]
impl<'a> RequestResetTokenProxyMaterial for ActiveRequestResetTokenProxyMaterial<'a> {
    type ProxyCall = TonicRequestResetTokenProxyCall<'a>;

    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

#[cfg(test)]
pub mod test {
    pub use super::proxy_call::test::*;

    use crate::auth::user::password::reset::request_token::proxy::action::RequestResetTokenProxyMaterial;

    pub struct StaticRequestResetTokenProxyMaterial;

    #[async_trait::async_trait]
    impl RequestResetTokenProxyMaterial for StaticRequestResetTokenProxyMaterial {
        type ProxyCall = StaticRequestResetTokenProxyCall;

        fn proxy_call(&self) -> &Self::ProxyCall {
            &StaticRequestResetTokenProxyCall
        }
    }
}
