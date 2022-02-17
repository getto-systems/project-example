use crate::z_lib::service::x_outside_feature::feature::GoogleServiceAuthorizerOutsideFeature;

pub struct CoreOutsideService {
    pub service_url: &'static str,
    pub google_authorizer: GoogleServiceAuthorizerOutsideFeature,
}
