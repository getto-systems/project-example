use std::sync::Arc;

use tonic::{metadata::MetadataMap, Request};

pub type AppData = Arc<AppFeature>;

pub struct AppFeature {}

impl AppFeature {
    pub async fn new() -> Self {
        Self {}
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
