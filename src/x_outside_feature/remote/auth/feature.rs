use std::sync::Arc;

use tonic::{metadata::MetadataMap, Request};

use super::env::AuthEnv;

use crate::auth::remote::x_outside_feature::auth::{
    feature::AuthOutsideFeature, init::new_auth_outside_feature,
};

pub struct AuthAppFeature {
    pub auth: AuthOutsideFeature,
}

impl AuthAppFeature {
    pub async fn new(env: &'static AuthEnv) -> Self {
        Self {
            auth: new_auth_outside_feature(env).await,
        }
    }
}

pub struct TonicRequest<T> {
    pub feature: Arc<AuthAppFeature>,
    pub metadata: MetadataMap,
    pub request: T,
}

pub fn extract_request<T>(request: Request<T>) -> TonicRequest<T> {
    let feature = request
        .extensions()
        .get::<Arc<AuthAppFeature>>()
        .expect("failed to get AppFeature")
        .clone();

    TonicRequest {
        feature,
        // metadata と inner の両方を into してくれるやつが無いため、to_owned する
        metadata: request.metadata().to_owned(),
        request: request.into_inner(),
    }
}
