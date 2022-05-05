use tonic::{metadata::errors::InvalidMetadataValue, Request};

use crate::x_content::metadata::{METADATA_NONCE, METADATA_REQUEST_ID, METADATA_TOKEN};

use crate::{
    auth::ticket::validate::infra::AuthMetadataContent, z_lib::service::infra::ServiceAuthorizer,
};

use crate::{auth::proxy::data::AuthProxyError, z_lib::service::data::ServiceAuthorizeError};

pub fn infra_error(label: &'static str, err: impl std::fmt::Display) -> AuthProxyError {
    AuthProxyError::InfraError(format!("proxy infra error; {}; {}", label, err))
}

pub enum AuthMetadataError {
    NonceNotFound,
    InvalidMetadataValue(InvalidMetadataValue),
    AuthorizeError(ServiceAuthorizeError),
}

impl std::fmt::Display for AuthMetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonceNotFound => write!(f, "nonce not found"),
            Self::InvalidMetadataValue(err) => write!(f, "invalid metadata value; {}", err),
            Self::AuthorizeError(err) => write!(f, "service authorize error; {}", err),
        }
    }
}

pub async fn set_metadata<T>(
    request: &mut Request<T>,
    request_id: &str,
    authorizer: &impl ServiceAuthorizer,
    metadata: AuthMetadataContent,
) -> Result<(), AuthMetadataError> {
    request.metadata_mut().insert(
        METADATA_REQUEST_ID,
        request_id
            .try_into()
            .map_err(AuthMetadataError::InvalidMetadataValue)?,
    );

    let nonce = metadata.nonce.ok_or(AuthMetadataError::NonceNotFound)?;
    request.metadata_mut().insert(
        METADATA_NONCE,
        nonce
            .extract()
            .try_into()
            .map_err(AuthMetadataError::InvalidMetadataValue)?,
    );

    // token は None のことがある
    if let Some(token) = metadata.token {
        request.metadata_mut().insert(
            METADATA_TOKEN,
            token
                .extract()
                .try_into()
                .map_err(AuthMetadataError::InvalidMetadataValue)?,
        );
    }

    if let Some(authorize_token) = authorizer
        .fetch_token()
        .await
        .map_err(AuthMetadataError::AuthorizeError)?
    {
        request.metadata_mut().insert(
            "authorization",
            format!("Bearer {}", authorize_token.extract())
                .try_into()
                .map_err(AuthMetadataError::InvalidMetadataValue)?,
        );
    }

    Ok(())
}
