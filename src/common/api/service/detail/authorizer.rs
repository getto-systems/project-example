use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Utc};
use reqwest::{Client, Url};

use crate::common::api::service::infra::ServiceAuthorizer;

use crate::common::api::service::data::{ServiceAuthorizeError, ServiceAuthorizeToken};

pub struct GoogleServiceAuthorizer {
    store: Arc<Mutex<GoogleServiceAuthorizerToken>>,
}

impl GoogleServiceAuthorizer {
    pub const fn new(store: Arc<Mutex<GoogleServiceAuthorizerToken>>) -> Self {
        Self { store }
    }
}

pub type GoogleServiceAuthorizerStore = Mutex<GoogleServiceAuthorizerToken>;
pub struct GoogleServiceAuthorizerToken(Option<TokenStore>);

impl Default for GoogleServiceAuthorizerToken {
    fn default() -> Self {
        Self(None)
    }
}

impl GoogleServiceAuthorizerToken {
    pub const fn new() -> Self {
        Self(None)
    }
}

struct TokenStore {
    token: ServiceAuthorizeToken,
    replaced_at: DateTime<Utc>,
}

impl TokenStore {
    fn has_expired(&self, now: DateTime<Utc>) -> bool {
        now - self.replaced_at > Duration::minutes(50)
    }
}

#[async_trait::async_trait]
impl ServiceAuthorizer for GoogleServiceAuthorizer {
    async fn fetch_token(
        &self,
        service_url: &str,
    ) -> Result<Option<ServiceAuthorizeToken>, ServiceAuthorizeError> {
        self.refresh(service_url).await
    }
}

impl GoogleServiceAuthorizer {
    async fn refresh(
        &self,
        service_url: &str,
    ) -> Result<Option<ServiceAuthorizeToken>, ServiceAuthorizeError> {
        if self.refresh_required(service_url)? {
            let token = self.request_token(service_url).await?;
            {
                let mut store = self.store.lock().unwrap();
                store.0.replace(TokenStore {
                    token,
                    replaced_at: Utc::now(),
                });
            }
        }

        let store = self.store.lock().unwrap();
        Ok(store.0.as_ref().map(|store| store.token.clone()))
    }
    fn refresh_required(&self, service_url: &str) -> Result<bool, ServiceAuthorizeError> {
        if Url::parse(service_url)
            .map_err(|err| infra_error("service url parse error", err))?
            .scheme()
            != "https"
        {
            return Ok(false);
        }

        let store = self.store.lock().unwrap();
        Ok(match store.0.as_ref() {
            None => true,
            Some(token) => token.has_expired(Utc::now()),
        })
    }

    async fn request_token(
        &self,
        service_url: &str,
    ) -> Result<ServiceAuthorizeToken, ServiceAuthorizeError> {
        let mut request_url = Url::parse("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/identity").map_err(|err| infra_error("request url parse error", err))?;
        request_url.set_query(Some(&format!("audience={}", service_url)));

        let request = Client::new()
            .get(request_url)
            .header("Metadata-Flavor", "Google");

        let response = request
            .send()
            .await
            .map_err(|err| infra_error("request token error", err))?;

        let token = response
            .text()
            .await
            .map_err(|err| infra_error("response parse error", err))?;

        Ok(ServiceAuthorizeToken::restore(token))
    }
}

fn infra_error(label: &'static str, err: impl std::fmt::Display) -> ServiceAuthorizeError {
    ServiceAuthorizeError::InfraError(format!("service infra error; {}; {}", label, err))
}
