use crate::x_outside_feature::_api::env::ApiEnv;

use super::feature::{ExampleOutsideFeature, ExampleOutsideService};

pub fn new_example_outside_feature(env: &'static ApiEnv) -> ExampleOutsideFeature {
    ExampleOutsideFeature {
        service: ExampleOutsideService {
            service_url: &env.domain_service_url,
        },
    }
}
