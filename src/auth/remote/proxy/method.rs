use tonic::{
    metadata::{errors::InvalidMetadataValue, MetadataValue},
    Request,
};

use crate::{
    auth::ticket::remote::kernel::x_tonic::metadata::{METADATA_NONCE, METADATA_TOKEN},
    x_outside_feature::remote::common::metadata::METADATA_REQUEST_ID,
    z_lib::remote::service::data::ServiceAuthorizeError,
};

use crate::{
    auth::ticket::remote::validate::infra::AuthMetadataContent,
    z_lib::remote::service::infra::ServiceAuthorizer,
};

pub enum AuthMetadataError {
    InvalidMetadataValue(InvalidMetadataValue),
    AuthorizeError(ServiceAuthorizeError),
    AuthorizeTokenNotFound,
    NonceNotFound,
}

impl std::fmt::Display for AuthMetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMetadataValue(err) => write!(f, "invalid metadata value; {}", err),
            Self::AuthorizeError(err) => write!(f, "service authorize error; {}", err),
            Self::AuthorizeTokenNotFound => write!(f, "service authorize token not found"),
            Self::NonceNotFound => write!(f, "nonce not found"),
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

    let authorize_token = authorizer
        .fetch_token()
        .await
        .map_err(AuthMetadataError::AuthorizeError)?
        .ok_or(AuthMetadataError::AuthorizeTokenNotFound)?;

    request.metadata_mut().insert(
        "authorization",
        MetadataValue::from_str(&format!("Bearer {}", authorize_token.extract()))
            .map_err(AuthMetadataError::InvalidMetadataValue)?,
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

    Ok(())
}
