use std::sync::Mutex;

use chrono::{DateTime, Duration, Utc};
use reqwest::{Client, Url};

use crate::z_lib::service::x_outside_feature::feature::GoogleServiceAuthorizerOutsideFeature;

use crate::z_lib::service::infra::ServiceAuthorizer;

use crate::z_lib::service::data::{ServiceAuthorizeError, ServiceAuthorizeToken};

pub struct GoogleServiceAuthorizer<'a> {
    service_url: &'static str,
    store: &'a GoogleServiceAuthorizerStore,
}

pub type GoogleServiceAuthorizerStore = Mutex<GoogleServiceAuthorizerToken>;
pub struct GoogleServiceAuthorizerToken(Option<TokenStore>);

impl GoogleServiceAuthorizerToken {
    pub const fn new() -> Self {
        Self(None)
    }

    pub fn to_store(self) -> GoogleServiceAuthorizerStore {
        Mutex::new(self)
    }
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
impl<'a> ServiceAuthorizer for GoogleServiceAuthorizer<'a> {
    async fn fetch_token(&self) -> Result<Option<ServiceAuthorizeToken>, ServiceAuthorizeError> {
        let url = Url::parse(self.service_url)
            .map_err(|err| infra_error("service url parse error", err))?;
        if url.scheme() == "https" {
            Ok(Some(self.refresh().await?))
        } else {
            Ok(None)
        }
    }
}

impl<'a> GoogleServiceAuthorizer<'a> {
    pub fn new(params: &'a GoogleServiceAuthorizerOutsideFeature) -> Self {
        Self {
            service_url: params.service_url,
            store: &params.store,
        }
    }

    async fn refresh(&self) -> Result<ServiceAuthorizeToken, ServiceAuthorizeError> {
        if self.refresh_required() {
            let token = self.request_token().await?;
            {
                let mut store = self.store.lock().unwrap();
                store.0.replace(TokenStore {
                    token,
                    fetched_at: Utc::now(),
                });
            }
        }

        let store = self.store.lock().unwrap();
        Ok(store.0.as_ref().unwrap().token.clone())
    }
    fn refresh_required(&self) -> bool {
        let store = self.store.lock().unwrap();
        match store.0.as_ref() {
            Some(token) => return token.has_expired(Utc::now()),
            None => return true,
        }
    }

    async fn request_token(&self) -> Result<ServiceAuthorizeToken, ServiceAuthorizeError> {
        let mut request_url = Url::parse("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/identity").map_err(|err| infra_error("request url parse error", err))?;
        request_url.set_query(Some(&format!("audience={}", self.service_url)));
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
