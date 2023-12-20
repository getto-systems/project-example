use tonic::metadata::MetadataMap;

use crate::x_content::metadata::{METADATA_AUTHENTICATE_TOKEN, METADATA_AUTHORIZE_TOKEN};

use crate::common::api::request::x_tonic::metadata::metadata;

use crate::auth::ticket::kernel::data::{
    AuthenticateToken, AuthenticateTokenExtract, AuthorizeToken, AuthorizeTokenExtract,
    ValidateAuthenticateTokenError, ValidateAuthorizeTokenError,
};

impl AuthenticateTokenExtract for &MetadataMap {
    fn convert(self) -> Result<AuthenticateToken, ValidateAuthenticateTokenError> {
        Ok(AuthenticateToken::restore(
            metadata(self, METADATA_AUTHENTICATE_TOKEN)
                .map_err(ValidateAuthenticateTokenError::MetadataError)?
                .ok_or(ValidateAuthenticateTokenError::NotFound)?
                .to_owned(),
        ))
    }
}

impl AuthorizeTokenExtract for &MetadataMap {
    fn convert(self) -> Result<AuthorizeToken, ValidateAuthorizeTokenError> {
        Ok(AuthorizeToken::restore(
            metadata(self, METADATA_AUTHORIZE_TOKEN)
                .map_err(ValidateAuthorizeTokenError::MetadataError)?
                .ok_or(ValidateAuthorizeTokenError::NotFound)?
                .to_owned(),
        ))
    }
}
