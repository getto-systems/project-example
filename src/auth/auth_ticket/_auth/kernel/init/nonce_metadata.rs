use tonic::metadata::MetadataMap;

use crate::auth::_common::metadata::METADATA_NONCE;

use crate::z_details::_common::request::x_tonic::metadata::metadata;

use crate::auth::auth_ticket::_common::kernel::infra::AuthNonceMetadata;

use crate::{
    auth::auth_ticket::_common::kernel::data::AuthNonce,
    z_details::_common::request::data::MetadataError,
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
        metadata(&self.metadata, METADATA_NONCE)
            .map(|value| value.map(|nonce| AuthNonce::restore(nonce.into())))
    }
}
