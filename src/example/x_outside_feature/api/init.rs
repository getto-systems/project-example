use crate::x_outside_feature::api::proxy::env::ProxyEnv;

use super::feature::{ExampleOutsideFeature, ExampleOutsideService};

pub fn new_example_outside_feature(env: &'static ProxyEnv) -> ExampleOutsideFeature {
    ExampleOutsideFeature {
        service: ExampleOutsideService {
            service_url: &env.domain_service_url,
        },
    }
}
