use actix_web::HttpRequest;
use tonic::metadata::MetadataMap;

use crate::auth::ticket::remote::kernel::{
    x_actix_web::header::{COOKIE_API_TOKEN, COOKIE_TICKET_TOKEN},
    x_tonic::metadata::METADATA_TOKEN,
};

use crate::z_lib::remote::request::{helper::cookie, x_tonic::metadata::metadata};

use crate::auth::ticket::remote::validate::infra::AuthTokenMetadata;

use crate::{
    auth::ticket::remote::kernel::data::AuthToken, z_lib::remote::request::data::MetadataError,
};

pub struct TicketTokenMetadata<'a> {
    request: &'a HttpRequest,
}

impl<'a> TicketTokenMetadata<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self { request }
    }
}

impl<'a> AuthTokenMetadata for TicketTokenMetadata<'a> {
    fn token(&self) -> Result<Option<AuthToken>, MetadataError> {
        Ok(cookie(&self.request, COOKIE_TICKET_TOKEN).map(AuthToken::restore))
    }
}

pub struct ApiTokenMetadata<'a> {
    request: &'a HttpRequest,
}

impl<'a> ApiTokenMetadata<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self { request }
    }
}

impl<'a> AuthTokenMetadata for ApiTokenMetadata<'a> {
    fn token(&self) -> Result<Option<AuthToken>, MetadataError> {
        Ok(cookie(&self.request, COOKIE_API_TOKEN).map(AuthToken::restore))
    }
}

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

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::remote::validate::infra::AuthTokenMetadata;

    use crate::{
        auth::ticket::remote::kernel::data::AuthToken, z_lib::remote::request::data::MetadataError,
    };

    pub struct StaticAuthTokenMetadata {
        token: AuthToken,
    }
    impl StaticAuthTokenMetadata {
        pub const fn new(token: String) -> Self {
            Self {
                token: AuthToken::restore(token),
            }
        }
    }

    impl AuthTokenMetadata for StaticAuthTokenMetadata {
        fn token(&self) -> Result<Option<AuthToken>, MetadataError> {
            Ok(Some(self.token.clone()))
        }
    }
}
