mod proxy_call;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::{
    auth::user::login_id::change::proxy::init::proxy_call::TonicOverwriteLoginIdProxyCall,
    common::proxy::init::ActiveCoreProxyMaterial,
};

use crate::{
    auth::user::login_id::change::action::OverwriteLoginIdActionInfo,
    common::proxy::action::CoreProxyAction,
};

pub type ActiveOverwriteLoginIdProxyMaterial<'a> =
    ActiveCoreProxyMaterial<'a, TonicOverwriteLoginIdProxyCall<'a>>;

impl<'a> ActiveOverwriteLoginIdProxyMaterial<'a> {
    pub fn action(feature: &'a ProxyAppFeature, request_id: RequestId) -> CoreProxyAction<Self> {
        CoreProxyAction::with_material(
            OverwriteLoginIdActionInfo.params(),
            ActiveCoreProxyMaterial::new(
                &feature.auth.decoding_key,
                TonicOverwriteLoginIdProxyCall::new(&feature.core, request_id),
            ),
        )
    }
}
