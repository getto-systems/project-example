mod proxy_call;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::{
    avail::unexpected_error::notify::proxy::init::proxy_call::TonicNotifyUnexpectedErrorProxyCall,
    common::proxy::init::ActiveCoreProxyMaterial,
};

use crate::{
    avail::unexpected_error::notify::action::NotifyUnexpectedErrorActionInfo,
    common::proxy::action::CoreProxyAction,
};

pub type ActiveNotifyUnexpectedErrorProxyMaterial<'a> =
    ActiveCoreProxyMaterial<'a, TonicNotifyUnexpectedErrorProxyCall<'a>>;

impl<'a> ActiveNotifyUnexpectedErrorProxyMaterial<'a> {
    pub fn action(feature: &'a ProxyAppFeature, request_id: RequestId) -> CoreProxyAction<Self> {
        CoreProxyAction::with_material(
            NotifyUnexpectedErrorActionInfo.params(),
            ActiveCoreProxyMaterial::new(
                &feature.auth.decoding_key,
                TonicNotifyUnexpectedErrorProxyCall::new(&feature.core, request_id),
            ),
        )
    }
}
