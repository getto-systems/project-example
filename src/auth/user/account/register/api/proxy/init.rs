mod proxy_call;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::auth::user::account::register::proxy::init::proxy_call::TonicRegisterAuthUserAccountProxyCall;

use crate::{
    auth::user::account::register::action::RegisterAuthUserAccountActionInfo,
    common::proxy::action::CoreProxyAction,
};

use crate::common::proxy::init::ActiveCoreProxyMaterial;

pub type ActiveRegisterAuthUserAccountProxyMaterial<'a> =
    ActiveCoreProxyMaterial<'a, TonicRegisterAuthUserAccountProxyCall<'a>>;

impl<'a> ActiveRegisterAuthUserAccountProxyMaterial<'a> {
    pub fn action(feature: &'a ProxyAppFeature, request_id: RequestId) -> CoreProxyAction<Self> {
        CoreProxyAction::with_material(
            RegisterAuthUserAccountActionInfo.params(),
            ActiveCoreProxyMaterial::new(
                &feature.auth.decoding_key,
                TonicRegisterAuthUserAccountProxyCall::new(&feature.core, request_id),
            ),
        )
    }
}
