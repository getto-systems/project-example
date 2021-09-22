use tonic::metadata::MetadataMap;

use crate::auth::_common::metadata::{METADATA_NONCE, METADATA_TOKEN};

use crate::z_details::_common::request::x_tonic::metadata::metadata;

use crate::auth::auth_ticket::_common::kernel::infra::{AuthMetadata, AuthMetadataContent};

use crate::{
    auth::auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
    z_details::_common::request::data::MetadataError,
};

pub struct TonicAuthMetadata<'a> {
    metadata: &'a MetadataMap,
}

impl<'a> TonicAuthMetadata<'a> {
    pub const fn new(metadata: &'a MetadataMap) -> Self {
        Self { metadata }
    }
}

impl<'a> AuthMetadata for TonicAuthMetadata<'a> {
    fn metadata(&self) -> Result<AuthMetadataContent, MetadataError> {
        Ok(AuthMetadataContent {
            nonce: fetch_metadata(&self.metadata, METADATA_NONCE, AuthNonce::restore)?,
            token: fetch_metadata(&self.metadata, METADATA_TOKEN, AuthToken::restore)?,
        })
    }
}

fn fetch_metadata<T>(
    map: &MetadataMap,
    key: &str,
    converter: impl Fn(String) -> T,
) -> Result<Option<T>, MetadataError> {
    metadata(map, key).map(|value| value.map(|value| converter(value.into())))
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_common::kernel::infra::{AuthMetadata, AuthMetadataContent};

    use crate::{
        auth::auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
        z_details::_common::request::data::MetadataError,
    };

    pub struct StaticAuthMetadata {
        pub nonce: String,
        pub token: String,
    }

    impl AuthMetadata for StaticAuthMetadata {
        fn metadata(&self) -> Result<AuthMetadataContent, MetadataError> {
            Ok(AuthMetadataContent {
                nonce: Some(AuthNonce::restore(self.nonce.clone())),
                token: Some(AuthToken::restore(self.token.clone())),
            })
        }
    }
}
