use tonic::metadata::MetadataMap;

use crate::auth::_common::metadata::METADATA_TOKEN;

use crate::z_details::_common::request::x_tonic::metadata::metadata;

use crate::auth::auth_ticket::remote::kernel::infra::AuthTokenMetadata;

use crate::{
    auth::auth_ticket::remote::kernel::data::AuthToken,
    z_details::_common::request::data::MetadataError,
};

pub struct TonicAuthTokenMetadata<'a> {
    metadata: &'a MetadataMap,
}

impl<'a> TonicAuthTokenMetadata<'a> {
    pub const fn new(metadata: &'a MetadataMap) -> Self {
        Self { metadata }
    }
}

impl<'a> AuthTokenMetadata for TonicAuthTokenMetadata<'a> {
    fn token(&self) -> Result<Option<AuthToken>, MetadataError> {
        metadata(&self.metadata, METADATA_TOKEN)
            .map(|value| value.map(|token| AuthToken::restore(token.into())))
    }
}
