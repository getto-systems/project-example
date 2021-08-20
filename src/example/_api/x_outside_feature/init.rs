use crate::x_outside_feature::_api::env::Env;

use super::feature::{ExampleOutsideFeature, ExampleOutsideService};

pub fn new_example_outside_feature(env: &'static Env) -> ExampleOutsideFeature {
    ExampleOutsideFeature {
        service: ExampleOutsideService {
            outline_service_url: &env.outline_service_url,
        },
    }
}
