use crate::{
    auth::auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
    z_details::_common::request::data::MetadataError,
};

pub trait AuthNonceMetadata {
    fn nonce(&self) -> Result<Option<AuthNonce>, MetadataError>;
}
pub trait AuthTokenMetadata {
    fn token(&self) -> Result<Option<AuthToken>, MetadataError>;
}
