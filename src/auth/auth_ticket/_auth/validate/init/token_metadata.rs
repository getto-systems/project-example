use tonic::metadata::MetadataMap;

use crate::auth::auth_ticket::_common::kernel::x_tonic::metadata::{
    METADATA_API_TOKEN, METADATA_TICKET_TOKEN,
};

use crate::z_details::_auth::request::helper::metadata;

use crate::auth::auth_ticket::_auth::validate::infra::AuthTokenMetadata;

use crate::{
    auth::auth_ticket::_common::kernel::data::AuthTokenValue,
    z_details::_auth::request::data::MetadataError,
};

pub struct TicketAuthTokenMetadata<'a> {
    metadata: &'a MetadataMap,
}

impl<'a> TicketAuthTokenMetadata<'a> {
    pub const fn new(metadata: &'a MetadataMap) -> Self {
        Self { metadata }
    }
}

impl<'a> AuthTokenMetadata for TicketAuthTokenMetadata<'a> {
    fn token(&self) -> Result<Option<AuthTokenValue>, MetadataError> {
        metadata(&self.metadata, METADATA_TICKET_TOKEN).map(|value| value.map(AuthTokenValue::new))
    }
}

pub struct ApiAuthTokenMetadata<'a> {
    metadata: &'a MetadataMap,
}

impl<'a> ApiAuthTokenMetadata<'a> {
    pub const fn new(metadata: &'a MetadataMap) -> Self {
        Self { metadata }
    }
}

impl<'a> AuthTokenMetadata for ApiAuthTokenMetadata<'a> {
    fn token(&self) -> Result<Option<AuthTokenValue>, MetadataError> {
        metadata(&self.metadata, METADATA_API_TOKEN).map(|value| value.map(AuthTokenValue::new))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_auth::validate::infra::AuthTokenMetadata;

    use crate::{
        auth::auth_ticket::_common::kernel::data::AuthTokenValue,
        z_details::_auth::request::data::MetadataError,
    };

    pub enum StaticAuthTokenMetadata {
        Valid(AuthTokenValue),
    }

    impl AuthTokenMetadata for StaticAuthTokenMetadata {
        fn token(&self) -> Result<Option<AuthTokenValue>, MetadataError> {
            match self {
                Self::Valid(token) => Ok(Some(token.clone())),
            }
        }
    }
}
