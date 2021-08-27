use std::sync::Mutex;

use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use tonic::{
    metadata::MetadataValue,
    transport::{Channel, ClientTlsConfig, Endpoint},
    Request,
};
use url::Url;

use crate::{
    auth::auth_ticket::_common::kernel::x_tonic::metadata::{METADATA_NONCE, METADATA_TOKEN},
    x_outside_feature::_common::metadata::METADATA_REQUEST_ID,
};

use crate::auth::{
    _api::service::data::AuthServiceError,
    auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
};

pub fn infra_error(err: impl std::fmt::Display) -> AuthServiceError {
    AuthServiceError::InfraError(format!("service infra error; {}", err))
}

pub fn new_endpoint(service_url: &'static str) -> Result<Endpoint, AuthServiceError> {
    let url = Url::parse(service_url).map_err(infra_error)?;
    if url.scheme() == "https" {
        let config = ClientTlsConfig::new().domain_name(
            url.host_str()
                .ok_or(AuthServiceError::InfraError("invalid service url".into()))?,
        );
        Channel::from_static(service_url)
            .tls_config(config)
            .map_err(infra_error)
    } else {
        Ok(Channel::from_static(service_url))
    }
}

pub struct AuthAuthorizer {
    service_url: &'static str,
    store: Mutex<Option<AuthIdToken>>,
}

struct AuthIdToken {
    token: String,
    fetched_at: DateTime<Utc>,
}

impl AuthIdToken {
    fn has_expired(&self, now: DateTime<Utc>) -> bool {
        now - self.fetched_at > Duration::minutes(50)
    }
}

impl AuthAuthorizer {
    pub fn new(service_url: &'static str) -> Self {
        Self {
            service_url,
            store: Mutex::new(None),
        }
    }

    pub async fn fetch_token(&self) -> Result<Option<String>, AuthServiceError> {
        let url = Url::parse(self.service_url).map_err(infra_error)?;
        if url.scheme() == "https" {
            Ok(Some(self.refresh().await?))
        } else {
            Ok(None)
        }
    }

    fn refresh_required(&self) -> bool {
        let store = self.store.lock().unwrap();
        match store.as_ref() {
            Some(token) => return token.has_expired(Utc::now()),
            None => return true,
        }
    }

    async fn refresh(&self) -> Result<String, AuthServiceError> {
        if self.refresh_required() {
            let token = self.request_token().await?;
            {
                let mut store = self.store.lock().unwrap();
                store.replace(AuthIdToken {
                    token,
                    fetched_at: Utc::now(),
                });
            }
        }

        let store = self.store.lock().unwrap();
        Ok(store.as_ref().unwrap().token.clone())
    }

    async fn request_token(&self) -> Result<String, AuthServiceError> {
        let mut request_url = Url::parse("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/identity").map_err(infra_error)?;
        request_url.set_query(Some(&format!("audience={}", self.service_url)));
        let request = Client::new()
            .get(request_url)
            .header("Metadata-Flavor", "Google");
        let response = request.send().await.map_err(infra_error)?;
        response.text().await.map_err(infra_error)
    }
}

pub fn set_authorization<T>(
    request: &mut Request<T>,
    token: Option<String>,
) -> Result<(), AuthServiceError> {
    if let Some(token) = token {
        request.metadata_mut().insert(
            "authorization",
            MetadataValue::from_str(&format!("Bearer {}", token)).map_err(infra_error)?,
        );
    }
    Ok(())
}

pub fn set_metadata<T>(
    request: &mut Request<T>,
    request_id: &str,
    nonce: Option<AuthNonce>,
    token: Option<AuthToken>,
) -> Result<(), AuthServiceError> {
    request.metadata_mut().insert(
        METADATA_REQUEST_ID,
        MetadataValue::from_str(request_id).map_err(infra_error)?,
    );
    if let Some(nonce) = nonce {
        request.metadata_mut().insert(
            METADATA_NONCE,
            MetadataValue::from_str(&nonce.extract()).map_err(infra_error)?,
        );
    }
    if let Some(token) = token {
        request.metadata_mut().insert(
            METADATA_TOKEN,
            MetadataValue::from_str(&token.extract()).map_err(infra_error)?,
        );
    }

    Ok(())
}
