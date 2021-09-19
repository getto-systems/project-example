use tonic::metadata::MetadataMap;

use crate::auth::_common::metadata::{METADATA_NONCE, METADATA_TOKEN};

use crate::z_details::_common::request::x_tonic::metadata::metadata;

use crate::auth::auth_ticket::_common::kernel::infra::{
    AuthServiceMetadata, AuthServiceMetadataContent,
};

use crate::auth::auth_ticket::_common::kernel::data::{
    AuthNonce, AuthServiceMetadataError, AuthToken,
};

pub struct TonicAuthServiceMetadata<'a> {
    metadata: &'a MetadataMap,
}

impl<'a> TonicAuthServiceMetadata<'a> {
    pub const fn new(metadata: &'a MetadataMap) -> Self {
        Self { metadata }
    }
}

impl<'a> AuthServiceMetadata for TonicAuthServiceMetadata<'a> {
    fn metadata(&self) -> Result<AuthServiceMetadataContent, AuthServiceMetadataError> {
        Ok(AuthServiceMetadataContent {
            nonce: fetch_metadata(&self.metadata, METADATA_NONCE, AuthNonce::restore)?,
            token: fetch_metadata(&self.metadata, METADATA_TOKEN, AuthToken::restore)?,
        })
    }
}

fn fetch_metadata<T>(
    map: &MetadataMap,
    key: &str,
    converter: impl Fn(String) -> T,
) -> Result<Option<T>, AuthServiceMetadataError> {
    metadata(map, key)
        .map(|value| value.map(|value| converter(value.into())))
        .map_err(AuthServiceMetadataError::MetadataError)
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_common::kernel::infra::{
        AuthServiceMetadata, AuthServiceMetadataContent,
    };

    use crate::auth::auth_ticket::_common::kernel::data::{
        AuthNonce, AuthServiceMetadataError, AuthToken,
    };

    pub struct StaticAuthServiceMetadata {
        pub nonce: String,
        pub token: String,
    }

    impl AuthServiceMetadata for StaticAuthServiceMetadata {
        fn metadata(&self) -> Result<AuthServiceMetadataContent, AuthServiceMetadataError> {
            Ok(AuthServiceMetadataContent {
                nonce: Some(AuthNonce::restore(self.nonce.clone())),
                token: Some(AuthToken::restore(self.token.clone())),
            })
        }
    }
}
