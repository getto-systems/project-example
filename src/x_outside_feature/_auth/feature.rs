use std::sync::Arc;

use tonic::{metadata::MetadataMap, Request};

use super::env::AuthEnv;

use crate::auth::_auth::x_outside_feature::{
    feature::AuthOutsideFeature, init::new_auth_outside_feature,
};

pub type AuthAppData = Arc<AuthAppFeature>;

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
    pub data: AuthAppData,
    pub metadata: MetadataMap,
    pub request: T,
}

pub fn extract_request<T>(request: Request<T>) -> TonicRequest<T> {
    let data = request
        .extensions()
        .get::<AuthAppData>()
        .expect("failed to get AppFeature")
        .clone();

    TonicRequest {
        data,
        // metadata と inner の両方を into してくれるやつが無いため、to_owned する
        metadata: request.metadata().to_owned(),
        request: request.into_inner(),
    }
}
