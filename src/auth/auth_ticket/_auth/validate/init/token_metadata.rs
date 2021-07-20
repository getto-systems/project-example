use tonic::Request;

use crate::auth::auth_ticket::_common::kernel::x_tonic::metadata::{
    METADATA_API_TOKEN, METADATA_TICKET_TOKEN,
};

use crate::z_details::_auth::request::helper::metadata;

use crate::auth::auth_ticket::_auth::validate::infra::AuthTokenMetadata;

use crate::auth::auth_ticket::_auth::kernel::data::AuthTokenValue;
use crate::z_details::_auth::request::data::MetadataError;

pub struct TicketAuthTokenMetadata<'a, T> {
    request: &'a Request<T>,
}

impl<'a, T> TicketAuthTokenMetadata<'a, T> {
    pub const fn new(request: &'a Request<T>) -> Self {
        Self { request }
    }
}

impl<'a, T> AuthTokenMetadata for TicketAuthTokenMetadata<'a, T> {
    fn token(&self) -> Result<AuthTokenValue, MetadataError> {
        metadata(&self.request, METADATA_TICKET_TOKEN).map(AuthTokenValue::new)
    }
}

pub struct ApiAuthTokenMetadata<'a, T> {
    request: &'a Request<T>,
}

impl<'a, T> ApiAuthTokenMetadata<'a, T> {
    pub const fn new(request: &'a Request<T>) -> Self {
        Self { request }
    }
}

impl<'a, T> AuthTokenMetadata for ApiAuthTokenMetadata<'a, T> {
    fn token(&self) -> Result<AuthTokenValue, MetadataError> {
        metadata(&self.request, METADATA_API_TOKEN).map(AuthTokenValue::new)
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_auth::validate::infra::AuthTokenMetadata;

    use crate::auth::auth_ticket::_auth::kernel::data::AuthTokenValue;
    use crate::z_details::_auth::request::data::MetadataError;

    pub enum StaticAuthTokenMetadata {
        Valid(AuthTokenValue),
        NotFound, // TODO これのテスト
    }

    impl AuthTokenMetadata for StaticAuthTokenMetadata {
        fn token(&self) -> Result<AuthTokenValue, MetadataError> {
            match self {
                Self::NotFound => Err(MetadataError::NotFound),
                Self::Valid(token) => Ok(token.clone()),
            }
        }
    }
}
