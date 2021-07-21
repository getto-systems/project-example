use tonic::metadata::MetadataMap;

use crate::auth::auth_ticket::_common::kernel::x_tonic::metadata::METADATA_NONCE;

use crate::z_details::_auth::request::helper::metadata;

use crate::auth::auth_ticket::_auth::kernel::infra::AuthNonceMetadata;

use crate::{
    auth::auth_ticket::_auth::kernel::data::AuthNonceValue,
    z_details::_auth::request::data::MetadataError,
};

pub struct TonicAuthNonceMetadata {
    metadata: MetadataMap,
}

impl TonicAuthNonceMetadata {
    pub const fn new(metadata: MetadataMap) -> Self {
        Self { metadata }
    }
}

impl AuthNonceMetadata for TonicAuthNonceMetadata {
    fn nonce(&self) -> Result<AuthNonceValue, MetadataError> {
        metadata(&self.metadata, METADATA_NONCE).map(AuthNonceValue::new)
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_auth::kernel::infra::AuthNonceMetadata;

    use crate::auth::auth_ticket::_auth::kernel::data::AuthNonceValue;
    use crate::z_details::_auth::request::data::MetadataError;

    pub enum StaticAuthNonceMetadata {
        Valid(AuthNonceValue),
    }

    impl AuthNonceMetadata for StaticAuthNonceMetadata {
        fn nonce(&self) -> Result<AuthNonceValue, MetadataError> {
            match self {
                Self::Valid(nonce) => Ok(nonce.clone()),
            }
        }
    }
}
