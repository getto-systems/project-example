mod proxy_call;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::{
    auth::user::account::search::proxy::init::proxy_call::TonicSearchAuthUserAccountProxyCall,
    common::proxy::init::ActiveCoreProxyMaterial,
};

use crate::{
    auth::user::account::search::action::SearchAuthUserAccountActionInfo,
    common::proxy::action::CoreProxyAction,
};

pub type ActiveSearchAuthUserAccountProxyMaterial<'a> =
    ActiveCoreProxyMaterial<'a, TonicSearchAuthUserAccountProxyCall<'a>>;

impl<'a> ActiveSearchAuthUserAccountProxyMaterial<'a> {
    pub fn action(feature: &'a ProxyAppFeature, request_id: RequestId) -> CoreProxyAction<Self> {
        CoreProxyAction::with_material(
            SearchAuthUserAccountActionInfo.params(),
            ActiveCoreProxyMaterial::new(
                &feature.auth.decoding_key,
                TonicSearchAuthUserAccountProxyCall::new(&feature.core, request_id),
            ),
        )
    }
}
