use crate::common::api::service::x_outside_feature::feature::GoogleServiceAuthorizerOutsideFeature;

pub struct CoreProxyOutsideFeature {
    pub service: CommonOutsideService,
}
pub struct CommonOutsideService {
    pub service_url: &'static str,
    pub google_authorizer: GoogleServiceAuthorizerOutsideFeature,
}

impl CommonOutsideService {
    pub fn new(service_url: &'static str) -> Self {
        Self {
            service_url,
            google_authorizer: GoogleServiceAuthorizerOutsideFeature::new(service_url),
        }
    }
}
