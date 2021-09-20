use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideKey;

use crate::auth::auth_ticket::{
    _api::kernel::init::{
        nonce_metadata::ActixWebAuthNonceMetadata,
        token_metadata::{ApiTokenMetadata, TicketTokenMetadata},
    },
    _common::kernel::init::token_decoder::{JwtApiTokenDecoder, JwtTicketTokenDecoder},
};

use crate::auth::auth_ticket::_common::kernel::infra::{
    AuthNonceMetadata, AuthMetadata, AuthMetadataContent, AuthTokenDecoder,
    AuthTokenMetadata,
};

use crate::auth::auth_ticket::_common::kernel::data::{
    AuthNonce, AuthMetadataError, AuthToken,
};

pub struct TicketAuthMetadata<'a> {
    nonce_metadata: ActixWebAuthNonceMetadata<'a>,
    token_metadata: TicketTokenMetadata<'a>,
    token_decoder: JwtTicketTokenDecoder<'a>,
}

impl<'a> TicketAuthMetadata<'a> {
    pub const fn new(key: &'a AuthOutsideKey, request: &'a HttpRequest) -> Self {
        Self {
            nonce_metadata: ActixWebAuthNonceMetadata::new(request),
            token_metadata: TicketTokenMetadata::new(request),
            token_decoder: JwtTicketTokenDecoder::new(&key.ticket),
        }
    }
}

impl<'a> AuthMetadata for TicketAuthMetadata<'a> {
    fn metadata(&self) -> Result<AuthMetadataContent, AuthMetadataError> {
        Ok(AuthMetadataContent {
            nonce: fetch_nonce(&self.nonce_metadata)?,
            token: decode_token(&self.token_metadata, &self.token_decoder)?,
        })
    }
}

pub struct ApiAuthMetadata<'a> {
    nonce_metadata: ActixWebAuthNonceMetadata<'a>,
    token_metadata: ApiTokenMetadata<'a>,
    token_decoder: JwtApiTokenDecoder<'a>,
}

impl<'a> ApiAuthMetadata<'a> {
    pub const fn new(key: &'a AuthOutsideKey, request: &'a HttpRequest) -> Self {
        Self {
            nonce_metadata: ActixWebAuthNonceMetadata::new(request),
            token_metadata: ApiTokenMetadata::new(request),
            token_decoder: JwtApiTokenDecoder::new(&key.api),
        }
    }
}

impl<'a> AuthMetadata for ApiAuthMetadata<'a> {
    fn metadata(&self) -> Result<AuthMetadataContent, AuthMetadataError> {
        Ok(AuthMetadataContent {
            nonce: fetch_nonce(&self.nonce_metadata)?,
            token: decode_token(&self.token_metadata, &self.token_decoder)?,
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
    fn metadata(&self) -> Result<AuthMetadataContent, AuthMetadataError> {
        Ok(AuthMetadataContent {
            nonce: fetch_nonce(&self.nonce_metadata)?,
            token: None,
        })
    }
}

fn fetch_nonce(
    nonce_metadata: &impl AuthNonceMetadata,
) -> Result<Option<AuthNonce>, AuthMetadataError> {
    nonce_metadata
        .nonce()
        .map_err(AuthMetadataError::MetadataError)
}
fn decode_token(
    token_metadata: &impl AuthTokenMetadata,
    token_decoder: &impl AuthTokenDecoder,
) -> Result<Option<AuthToken>, AuthMetadataError> {
    match token_metadata
        .token()
        .map_err(AuthMetadataError::MetadataError)?
    {
        None => Ok(None),
        Some(token) => {
            token_decoder
                .decode(&token)
                .map_err(AuthMetadataError::DecodeError)?;
            Ok(Some(token))
        }
    }
}
