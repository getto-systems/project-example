use std::sync::Mutex;

use crate::common::api::service::detail::authorizer::{
    GoogleServiceAuthorizerStore, GoogleServiceAuthorizerToken,
};

pub struct GoogleServiceAuthorizerOutsideFeature {
    pub service_url: &'static str,
    pub store: GoogleServiceAuthorizerStore,
}

impl GoogleServiceAuthorizerOutsideFeature {
    pub fn new(service_url: &'static str) -> Self {
        Self {
            service_url,
            store: Mutex::new(GoogleServiceAuthorizerToken::new()),
        }
    }
}
