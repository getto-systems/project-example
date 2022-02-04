use crate::x_outside_feature::api::proxy::env::ProxyEnv;

use super::feature::{CoreOutsideFeature, CoreOutsideService};

pub fn new_core_outside_feature(env: &'static ProxyEnv) -> CoreOutsideFeature {
    CoreOutsideFeature {
        service: CoreOutsideService {
            service_url: &env.domain_service_url,
        },
    }
}
