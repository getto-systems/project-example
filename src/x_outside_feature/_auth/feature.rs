use std::sync::Arc;

use tonic::{metadata::MetadataMap, Extensions, Request};

use super::env::Env;

use crate::auth::_auth::x_outside_feature::{
    feature::AuthOutsideFeature, init::new_auth_outside_feature,
};

pub type AppData = Arc<AppFeature>;

pub struct AppFeature {
    pub auth: AuthOutsideFeature,
}

impl AppFeature {
    pub async fn new(env: &'static Env) -> Self {
        Self {
            auth: new_auth_outside_feature(env).await,
        }
    }
}

pub struct TonicRequest<T> {
    pub data: AppData,
    pub metadata: MetadataMap,
    pub request: T,
}

pub fn extract_request<T>(request: Request<T>) -> TonicRequest<T> {
    let data = request
        .extensions()
        .get::<AppData>()
        .expect("failed to get AppFeature")
        .clone();

    TonicRequest {
        data,
        // metadata と inner の両方を into してくれるやつが無いため、to_owned する
        metadata: request.metadata().to_owned(),
        request: request.into_inner(),
    }
}

pub fn app_data(extensions: &Extensions) -> &AppData {
    extensions
        .get::<AppData>()
        .expect("failed to get AppFeature")
}
