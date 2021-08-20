use crate::x_outside_feature::_api::env::Env;

use super::feature::{OutlineOutsideFeature, OutlineOutsideService};

pub fn new_outline_outside_feature(env: &'static Env) -> OutlineOutsideFeature {
    OutlineOutsideFeature {
        service: OutlineOutsideService {
            outline_service_url: &env.outline_service_url,
        },
    }
}
