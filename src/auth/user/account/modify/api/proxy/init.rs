mod proxy_call;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::{
    auth::user::account::modify::proxy::init::proxy_call::TonicModifyAuthUserAccountProxyCall,
    common::proxy::init::ActiveCoreProxyMaterial,
};

use crate::{
    auth::user::account::modify::action::ModifyAuthUserAccountActionInfo,
    common::proxy::action::CoreProxyAction,
};

pub type ActiveModifyAuthUserAccountProxyMaterial<'a> =
    ActiveCoreProxyMaterial<'a, TonicModifyAuthUserAccountProxyCall<'a>>;

impl<'a> ActiveModifyAuthUserAccountProxyMaterial<'a> {
    pub fn action(feature: &'a ProxyAppFeature, request_id: RequestId) -> CoreProxyAction<Self> {
        CoreProxyAction::with_material(
            ModifyAuthUserAccountActionInfo.params(),
            ActiveCoreProxyMaterial::new(
                &feature.auth.decoding_key,
                TonicModifyAuthUserAccountProxyCall::new(&feature.core, request_id),
            ),
        )
    }
}
