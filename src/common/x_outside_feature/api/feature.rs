use std::sync::{Arc, Mutex};

use crate::common::api::service::detail::authorizer::GoogleServiceAuthorizerToken;

pub struct CoreProxyOutsideFeature {
    pub service_url: &'static str,
    pub google_authorize_store: Arc<Mutex<GoogleServiceAuthorizerToken>>,
}
