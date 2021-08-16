use tonic::metadata::MetadataMap;

use crate::auth::auth_ticket::_common::kernel::x_tonic::metadata::METADATA_NONCE;

use crate::z_details::_auth::request::helper::metadata;

use crate::auth::auth_ticket::_auth::kernel::infra::AuthNonceMetadata;

use crate::{
    auth::auth_ticket::_common::kernel::data::AuthNonce,
    z_details::_auth::request::data::MetadataError,
};

pub struct TonicAuthNonceMetadata<'a> {
    metadata: &'a MetadataMap,
}

impl<'a> TonicAuthNonceMetadata<'a> {
    pub const fn new(metadata: &'a MetadataMap) -> Self {
        Self { metadata }
    }
}

impl<'a> AuthNonceMetadata for TonicAuthNonceMetadata<'a> {
    fn nonce(&self) -> Result<Option<AuthNonce>, MetadataError> {
        metadata(&self.metadata, METADATA_NONCE).map(|value| value.map(AuthNonce::new))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_auth::kernel::infra::AuthNonceMetadata;

    use crate::auth::auth_ticket::_common::kernel::data::AuthNonce;
    use crate::z_details::_auth::request::data::MetadataError;

    pub enum StaticAuthNonceMetadata {
        Valid(AuthNonce),
    }

    impl AuthNonceMetadata for StaticAuthNonceMetadata {
        fn nonce(&self) -> Result<Option<AuthNonce>, MetadataError> {
            match self {
                Self::Valid(nonce) => Ok(Some(nonce.clone())),
            }
        }
    }
}
