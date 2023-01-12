use std::collections::{HashMap, HashSet};

use crate::x_content::metadata::{METADATA_AUTHENTICATE_TOKEN, METADATA_AUTHORIZE_TOKEN};

use crate::x_content::permission::AuthPermission;
use crate::{
    auth::ticket::kernel::data::{
        AuthPermissionGranted, AuthPermissionGrantedExtract, AuthenticateToken, AuthorizeToken,
        ValidateAuthPermissionGrantedError,
    },
    common::api::request::data::MetadataError,
    common::proxy::data::ProxyMetadataExtract,
};

impl AuthPermissionGrantedExtract for Vec<String> {
    fn convert(self) -> Result<AuthPermissionGranted, ValidateAuthPermissionGrantedError> {
        let mut permissions = HashSet::new();
        for permission in self {
            permissions.insert(
                AuthPermission::convert(permission)
                    .ok_or(ValidateAuthPermissionGrantedError::InvalidPermission)?,
            );
        }
        Ok(AuthPermissionGranted::restore(permissions))
    }
}

impl ProxyMetadataExtract for AuthenticateToken {
    fn convert(self) -> Result<HashMap<&'static str, String>, MetadataError> {
        Ok(vec![(METADATA_AUTHENTICATE_TOKEN, self.extract())]
            .into_iter()
            .collect())
    }
}

impl ProxyMetadataExtract for AuthorizeToken {
    fn convert(self) -> Result<HashMap<&'static str, String>, MetadataError> {
        Ok(vec![(METADATA_AUTHORIZE_TOKEN, self.extract())]
            .into_iter()
            .collect())
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::kernel::data::{
        AuthenticateToken, AuthenticateTokenExtract, AuthorizeToken, AuthorizeTokenExtract,
        ValidateAuthenticateTokenError, ValidateAuthorizeTokenError,
    };

    pub struct StaticAuthenticateToken;

    impl AuthenticateTokenExtract for StaticAuthenticateToken {
        fn convert(self) -> Result<AuthenticateToken, ValidateAuthenticateTokenError> {
            Ok(AuthenticateToken::restore("TOKEN".to_owned()))
        }
    }

    pub struct StaticAuthorizeToken;

    impl AuthorizeTokenExtract for StaticAuthorizeToken {
        fn convert(self) -> Result<AuthorizeToken, ValidateAuthorizeTokenError> {
            Ok(AuthorizeToken::restore("TOKEN".to_owned()))
        }
    }
}
