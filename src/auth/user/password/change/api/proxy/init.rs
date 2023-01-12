mod change_password;
mod overwrite_password;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::{
    auth::user::password::change::proxy::init::{
        change_password::TonicChangePasswordProxyCall,
        overwrite_password::TonicOverwritePasswordProxyCall,
    },
    common::proxy::init::ActiveCoreProxyMaterial,
};

use crate::{
    auth::user::password::change::action::{ChangePasswordActionInfo, OverwritePasswordActionInfo},
    common::proxy::action::CoreProxyAction,
};

pub type ActiveChangePasswordProxyMaterial<'a> =
    ActiveCoreProxyMaterial<'a, TonicChangePasswordProxyCall<'a>>;

impl<'a> ActiveChangePasswordProxyMaterial<'a> {
    pub fn action(feature: &'a ProxyAppFeature, request_id: RequestId) -> CoreProxyAction<Self> {
        CoreProxyAction::with_material(
            ChangePasswordActionInfo.params(),
            ActiveCoreProxyMaterial::new(
                &feature.auth.decoding_key,
                TonicChangePasswordProxyCall::new(&feature.core, request_id),
            ),
        )
    }
}

pub type ActiveOverwritePasswordProxyMaterial<'a> =
    ActiveCoreProxyMaterial<'a, TonicOverwritePasswordProxyCall<'a>>;

impl<'a> ActiveOverwritePasswordProxyMaterial<'a> {
    pub fn action(feature: &'a ProxyAppFeature, request_id: RequestId) -> CoreProxyAction<Self> {
        CoreProxyAction::with_material(
            OverwritePasswordActionInfo.params(),
            ActiveCoreProxyMaterial::new(
                &feature.auth.decoding_key,
                TonicOverwritePasswordProxyCall::new(&feature.core, request_id),
            ),
        )
    }
}
