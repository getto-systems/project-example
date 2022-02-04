use std::sync::Arc;

use tonic::{metadata::MetadataMap, Request};

use crate::{
    auth::x_outside_feature::api::core::{
        feature::AuthOutsideFeature, init::new_auth_outside_feature,
    },
    x_outside_feature::api::example::env::ExampleEnv,
};

pub struct ExampleAppFeature {
    pub auth: AuthOutsideFeature,
}

impl ExampleAppFeature {
    pub async fn new(env: &'static ExampleEnv) -> Self {
        Self {
            auth: new_auth_outside_feature(env).await,
        }
    }
}

pub struct TonicRequest<T> {
    pub feature: Arc<ExampleAppFeature>,
    pub metadata: MetadataMap,
    pub request: T,
}

pub fn extract_request<T>(request: Request<T>) -> TonicRequest<T> {
    let feature = request
        .extensions()
        .get::<Arc<ExampleAppFeature>>()
        .expect("failed to get AppFeature")
        .clone();

    TonicRequest {
        feature,
        // metadata と inner の両方を into してくれるやつが無いため、to_owned する
        metadata: request.metadata().to_owned(),
        request: request.into_inner(),
    }
}
