use tonic::metadata::MetadataMap;

use crate::auth::auth_ticket::_common::kernel::x_tonic::metadata::METADATA_TOKEN;

use crate::z_details::_auth::request::helper::metadata;

use crate::auth::auth_ticket::_auth::validate::infra::AuthTokenMetadata;

use crate::{
    auth::auth_ticket::_common::kernel::data::AuthToken,
    z_details::_auth::request::data::MetadataError,
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
        metadata(&self.metadata, METADATA_TOKEN).map(|value| value.map(AuthToken::new))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_auth::validate::infra::AuthTokenMetadata;

    use crate::{
        auth::auth_ticket::_common::kernel::data::AuthToken,
        z_details::_auth::request::data::MetadataError,
    };

    pub enum StaticAuthTokenMetadata {
        Valid(AuthToken),
    }

    impl AuthTokenMetadata for StaticAuthTokenMetadata {
        fn token(&self) -> Result<Option<AuthToken>, MetadataError> {
            match self {
                Self::Valid(token) => Ok(Some(token.clone())),
            }
        }
    }
}
