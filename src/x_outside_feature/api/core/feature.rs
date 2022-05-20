use std::sync::Arc;

use tonic::{metadata::MetadataMap, Request};

use crate::{
    auth::x_outside_feature::feature::AuthOutsideService, x_outside_feature::core::env::CoreEnv,
    z_lib::service::x_outside_feature::feature::GoogleServiceAuthorizerOutsideFeature,
};

pub struct CoreAppFeature {
    pub auth: AuthOutsideFeature,
}

pub struct AuthOutsideFeature {
    pub service: AuthOutsideService,
}

impl CoreAppFeature {
    pub fn new(env: &'static CoreEnv) -> Self {
        Self {
            auth: AuthOutsideFeature {
                service: AuthOutsideService {
                    service_url: &env.auth_service_url,
                    google_authorizer: GoogleServiceAuthorizerOutsideFeature::new(
                        &env.auth_service_url,
                    ),
                },
            },
        }
    }
}

pub struct CoreTonicRequest<T> {
    pub feature: Arc<CoreAppFeature>,
    pub metadata: MetadataMap,
    pub request: T,
}

pub fn extract_core_request<T>(request: Request<T>) -> CoreTonicRequest<T> {
    let feature = request
        .extensions()
        .get::<Arc<CoreAppFeature>>()
        .expect("failed to get AppFeature")
        .clone();

    CoreTonicRequest {
        feature,
        // metadata と inner の両方を into してくれるやつが無いため、to_owned する
        metadata: request.metadata().to_owned(),
        request: request.into_inner(),
    }
}
