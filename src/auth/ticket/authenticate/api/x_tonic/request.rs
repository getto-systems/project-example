use tonic::metadata::MetadataMap;

use crate::common::api::request::x_tonic::metadata::metadata;

use crate::x_content::metadata::METADATA_AUTHENTICATE_TOKEN;

use crate::auth::ticket::kernel::data::{
    AuthenticateToken, AuthenticateTokenExtract, ValidateAuthenticateTokenError,
};

impl AuthenticateTokenExtract for &MetadataMap {
    fn convert(self) -> Result<AuthenticateToken, ValidateAuthenticateTokenError> {
        type Error = ValidateAuthenticateTokenError;
        Ok(AuthenticateToken::restore(
            metadata(self, METADATA_AUTHENTICATE_TOKEN)
                .map_err(Error::MetadataError)?
                .ok_or(Error::NotFound)?
                .to_owned(),
        ))
    }
}
