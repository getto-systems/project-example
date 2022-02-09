use std::sync::Mutex;

use chrono::{DateTime, Duration, Utc};
use reqwest::{Client, Url};

use crate::z_lib::service::{
    data::{ServiceAuthorizeError, ServiceAuthorizeToken},
    helper::infra_error,
    infra::ServiceAuthorizer,
};

pub struct GoogleServiceAuthorizer {
    service_url: &'static str,
    store: Mutex<Option<TokenStore>>,
}

struct TokenStore {
    token: ServiceAuthorizeToken,
    fetched_at: DateTime<Utc>,
}

impl TokenStore {
    fn has_expired(&self, now: DateTime<Utc>) -> bool {
        now - self.fetched_at > Duration::minutes(50)
    }
}

#[async_trait::async_trait]
impl ServiceAuthorizer for GoogleServiceAuthorizer {
    async fn fetch_token(&self) -> Result<Option<ServiceAuthorizeToken>, ServiceAuthorizeError> {
        let url = Url::parse(self.service_url).map_err(infra_error)?;
        if url.scheme() == "https" {
            Ok(Some(self.refresh().await?))
        } else {
            Ok(None)
        }
    }
}

impl GoogleServiceAuthorizer {
    pub fn new(service_url: &'static str) -> Self {
        Self {
            service_url,
            store: Mutex::new(None),
        }
    }

    async fn refresh(&self) -> Result<ServiceAuthorizeToken, ServiceAuthorizeError> {
        if self.refresh_required() {
            let token = self.request_token().await?;
            {
                let mut store = self.store.lock().unwrap();
                store.replace(TokenStore {
                    token,
                    fetched_at: Utc::now(),
                });
            }
        }

        let store = self.store.lock().unwrap();
        Ok(store.as_ref().unwrap().token.clone())
    }
    fn refresh_required(&self) -> bool {
        let store = self.store.lock().unwrap();
        match store.as_ref() {
            Some(token) => return token.has_expired(Utc::now()),
            None => return true,
        }
    }

    async fn request_token(&self) -> Result<ServiceAuthorizeToken, ServiceAuthorizeError> {
        let mut request_url = Url::parse("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/identity").map_err(infra_error)?;
        request_url.set_query(Some(&format!("audience={}", self.service_url)));
        let request = Client::new()
            .get(request_url)
            .header("Metadata-Flavor", "Google");
        let response = request.send().await.map_err(infra_error)?;
        let token = response.text().await.map_err(infra_error)?;
        Ok(ServiceAuthorizeToken::restore(token))
    }
}
