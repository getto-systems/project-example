use actix_web::HttpRequest;

use crate::auth::auth_ticket::_api::kernel::init::{
    nonce_metadata::ActixWebAuthNonceMetadata,
    token_metadata::{ApiTokenMetadata, TicketTokenMetadata},
};

use crate::auth::auth_ticket::_common::kernel::infra::{
    AuthMetadata, AuthMetadataContent, AuthNonceMetadata, AuthTokenMetadata,
};

use crate::z_details::_common::request::data::MetadataError;

pub struct TicketAuthMetadata<'a> {
    nonce_metadata: ActixWebAuthNonceMetadata<'a>,
    token_metadata: TicketTokenMetadata<'a>,
}

impl<'a> TicketAuthMetadata<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self {
            nonce_metadata: ActixWebAuthNonceMetadata::new(request),
            token_metadata: TicketTokenMetadata::new(request),
        }
    }
}

impl<'a> AuthMetadata for TicketAuthMetadata<'a> {
    fn metadata(&self) -> Result<AuthMetadataContent, MetadataError> {
        Ok(AuthMetadataContent {
            nonce: self.nonce_metadata.nonce()?,
            token: self.token_metadata.token()?,
        })
    }
}

pub struct ApiAuthMetadata<'a> {
    nonce_metadata: ActixWebAuthNonceMetadata<'a>,
    token_metadata: ApiTokenMetadata<'a>,
}

impl<'a> ApiAuthMetadata<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self {
            nonce_metadata: ActixWebAuthNonceMetadata::new(request),
            token_metadata: ApiTokenMetadata::new(request),
        }
    }
}

impl<'a> AuthMetadata for ApiAuthMetadata<'a> {
    fn metadata(&self) -> Result<AuthMetadataContent, MetadataError> {
        Ok(AuthMetadataContent {
            nonce: self.nonce_metadata.nonce()?,
            token: self.token_metadata.token()?,
        })
    }
}

pub struct NoAuthMetadata<'a> {
    nonce_metadata: ActixWebAuthNonceMetadata<'a>,
}

impl<'a> NoAuthMetadata<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self {
            nonce_metadata: ActixWebAuthNonceMetadata::new(request),
        }
    }
}

impl<'a> AuthMetadata for NoAuthMetadata<'a> {
    fn metadata(&self) -> Result<AuthMetadataContent, MetadataError> {
        Ok(AuthMetadataContent {
            nonce: self.nonce_metadata.nonce()?,
            token: None,
        })
    }
}
