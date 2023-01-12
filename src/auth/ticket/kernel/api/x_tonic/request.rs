use tonic::metadata::MetadataMap;

use crate::x_content::metadata::METADATA_AUTHORIZE_TOKEN;

use crate::common::api::request::x_tonic::metadata::metadata;

use crate::auth::ticket::kernel::data::{
    AuthorizeToken, AuthorizeTokenExtract, ValidateAuthorizeTokenError,
};

impl AuthorizeTokenExtract for &MetadataMap {
    fn convert(self) -> Result<AuthorizeToken, ValidateAuthorizeTokenError> {
        type Error = ValidateAuthorizeTokenError;
        Ok(AuthorizeToken::restore(
            metadata(self, METADATA_AUTHORIZE_TOKEN)
                .map_err(ValidateAuthorizeTokenError::MetadataError)?
                .ok_or(Error::NotFound)?
                .to_owned(),
        ))
    }
}
