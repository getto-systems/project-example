mod proxy_call;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::{
    auth::user::password::reset::token_destination::change::proxy::init::proxy_call::TonicChangeResetTokenDestinationProxyCall,
    common::proxy::init::ActiveCoreProxyMaterial,
};

use crate::{
    auth::user::password::reset::token_destination::change::action::ChangeResetTokenDestinationActionInfo,
    common::proxy::action::CoreProxyAction,
};

pub type ActiveChangeResetTokenDestinationProxyMaterial<'a> =
    ActiveCoreProxyMaterial<'a, TonicChangeResetTokenDestinationProxyCall<'a>>;

impl<'a> ActiveChangeResetTokenDestinationProxyMaterial<'a> {
    pub fn action(feature: &'a ProxyAppFeature, request_id: RequestId) -> CoreProxyAction<Self> {
        CoreProxyAction::with_material(
            ChangeResetTokenDestinationActionInfo.params(),
            ActiveCoreProxyMaterial::new(
                &feature.auth.decoding_key,
                TonicChangeResetTokenDestinationProxyCall::new(&feature.core, request_id),
            ),
        )
    }
}
