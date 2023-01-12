mod proxy_call;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::{
    auth::user::account::unregister::proxy::init::proxy_call::TonicUnregisterAuthUserAccountProxyCall,
    common::proxy::init::ActiveCoreProxyMaterial,
};

use crate::{
    auth::user::account::unregister::action::UnregisterAuthUserAccountActionInfo,
    common::proxy::action::CoreProxyAction,
};

pub type ActiveUnregisterAuthUserAccountProxyMaterial<'a> =
    ActiveCoreProxyMaterial<'a, TonicUnregisterAuthUserAccountProxyCall<'a>>;

impl<'a> ActiveUnregisterAuthUserAccountProxyMaterial<'a> {
    pub fn action(feature: &'a ProxyAppFeature, request_id: RequestId) -> CoreProxyAction<Self> {
        CoreProxyAction::with_material(
            UnregisterAuthUserAccountActionInfo.params(),
            ActiveCoreProxyMaterial::new(
                &feature.auth.decoding_key,
                TonicUnregisterAuthUserAccountProxyCall::new(&feature.core, request_id),
            ),
        )
    }
}
