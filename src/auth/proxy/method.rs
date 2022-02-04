use tonic::{
    metadata::{errors::InvalidMetadataValue, MetadataValue},
    Request,
};

use crate::{
    auth::ticket::kernel::api::x_tonic::metadata::{METADATA_NONCE, METADATA_TOKEN},
    x_outside_feature::api::common::metadata::METADATA_REQUEST_ID,
};

use crate::{
    auth::ticket::validate::infra::AuthMetadataContent,
    z_lib::api::service::infra::ServiceAuthorizer,
};

use crate::z_lib::api::service::data::ServiceAuthorizeError;

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
        MetadataValue::from_str(request_id).map_err(AuthMetadataError::InvalidMetadataValue)?,
    );

    let nonce = metadata.nonce.ok_or(AuthMetadataError::NonceNotFound)?;
    request.metadata_mut().insert(
        METADATA_NONCE,
        MetadataValue::from_str(&nonce.extract())
            .map_err(AuthMetadataError::InvalidMetadataValue)?,
    );

    // token は None のことがある
    if let Some(token) = metadata.token {
        request.metadata_mut().insert(
            METADATA_TOKEN,
            MetadataValue::from_str(&token.extract())
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
            MetadataValue::from_str(&format!("Bearer {}", authorize_token.extract()))
                .map_err(AuthMetadataError::InvalidMetadataValue)?,
        );
    }

    Ok(())
}
