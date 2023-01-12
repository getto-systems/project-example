use std::sync::Arc;

use tonic::{metadata::MetadataMap, Request};

use crate::{
    auth::x_outside_feature::feature::{AuthOutsideAuthorizeKey, AuthServiceOutsideFeature},
    common::x_outside_feature::feature::CommonOutsideService,
    x_outside_feature::{core::env::CoreEnv, data::RequestId},
};

use crate::common::api::jwt::helper::decoding_key_from_ec_pem;

use crate::common::api::logger::infra::LogOutputLevel;

pub struct CoreAppFeature {
    pub log_level: LogOutputLevel,
    pub auth: AuthServiceOutsideFeature,
}

impl CoreAppFeature {
    pub fn new(env: &'static CoreEnv) -> Self {
        Self {
            log_level: LogOutputLevel::parse(&env.log_level),
            auth: AuthServiceOutsideFeature {
                service: CommonOutsideService::new(&env.auth_service_url),
                decoding_key: AuthOutsideAuthorizeKey {
                    key: decoding_key_from_ec_pem(&env.authorize_public_key),
                },
            },
        }
    }
}

pub struct CoreTonicRequest<T> {
    pub feature: Arc<CoreAppFeature>,
    pub metadata: MetadataMap,
    pub request: T,
    pub request_id: RequestId,
}

impl<T> CoreTonicRequest<T> {
    pub fn from_request(request: Request<T>) -> Self {
        let feature = Arc::clone(
            request
                .extensions()
                .get::<Arc<CoreAppFeature>>()
                .expect("failed to get AppFeature"),
        );

        // metadata と inner の両方を into してくれるやつが無いため、to_owned する
        let metadata = request.metadata().to_owned();
        let request_id = RequestId::from_metadata(&metadata);

        CoreTonicRequest {
            feature,
            metadata,
            request: request.into_inner(),
            request_id,
        }
    }
}
