mod proxy_call;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::common::{
    outline::load::proxy::init::proxy_call::TonicGetOutlineMenuBadgeProxyCall,
    proxy::init::ActiveCoreProxyMaterial,
};

use crate::common::{
    outline::load::action::LoadOutlineMenuBadgeActionInfo, proxy::action::CoreProxyAction,
};

pub type ActiveGetOutlineMenuBadgeProxyMaterial<'a> =
    ActiveCoreProxyMaterial<'a, TonicGetOutlineMenuBadgeProxyCall<'a>>;

impl<'a> ActiveGetOutlineMenuBadgeProxyMaterial<'a> {
    pub fn action(feature: &'a ProxyAppFeature, request_id: RequestId) -> CoreProxyAction<Self> {
        CoreProxyAction::with_material(
            LoadOutlineMenuBadgeActionInfo.params(),
            ActiveCoreProxyMaterial::new(
                &feature.auth.decoding_key,
                TonicGetOutlineMenuBadgeProxyCall::new(&feature.core, request_id),
            ),
        )
    }
}
