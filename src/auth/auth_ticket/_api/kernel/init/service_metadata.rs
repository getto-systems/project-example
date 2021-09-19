use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideKey;

use crate::auth::auth_ticket::{
    _api::kernel::init::{
        nonce_metadata::ActixWebAuthNonceMetadata,
        token_metadata::{ApiAuthTokenMetadata, TicketAuthTokenMetadata},
    },
    _common::kernel::init::token_decoder::{JwtApiTokenDecoder, JwtTicketTokenDecoder},
};

use crate::auth::auth_ticket::_common::kernel::infra::{
    AuthNonceMetadata, AuthServiceMetadata, AuthServiceMetadataContent, AuthTokenDecoder,
    AuthTokenMetadata,
};

use crate::auth::auth_ticket::_common::kernel::data::{
    AuthNonce, AuthServiceMetadataError, AuthToken,
};

pub struct TicketServiceMetadata<'a> {
    nonce_metadata: ActixWebAuthNonceMetadata<'a>,
    token_metadata: TicketAuthTokenMetadata<'a>,
    token_decoder: JwtTicketTokenDecoder<'a>,
}

impl<'a> TicketServiceMetadata<'a> {
    pub const fn new(request: &'a HttpRequest, key: &'a AuthOutsideKey) -> Self {
        Self {
            nonce_metadata: ActixWebAuthNonceMetadata::new(request),
            token_metadata: TicketAuthTokenMetadata::new(request),
            token_decoder: JwtTicketTokenDecoder::new(&key.ticket),
        }
    }
}

impl<'a> AuthServiceMetadata for TicketServiceMetadata<'a> {
    fn metadata(&self) -> Result<AuthServiceMetadataContent, AuthServiceMetadataError> {
        Ok(AuthServiceMetadataContent {
            nonce: fetch_nonce(&self.nonce_metadata)?,
            token: decode_token(&self.token_metadata, &self.token_decoder)?,
        })
    }
}

pub struct ApiServiceMetadata<'a> {
    nonce_metadata: ActixWebAuthNonceMetadata<'a>,
    token_metadata: ApiAuthTokenMetadata<'a>,
    token_decoder: JwtApiTokenDecoder<'a>,
}

impl<'a> ApiServiceMetadata<'a> {
    pub const fn new(request: &'a HttpRequest, key: &'a AuthOutsideKey) -> Self {
        Self {
            nonce_metadata: ActixWebAuthNonceMetadata::new(request),
            token_metadata: ApiAuthTokenMetadata::new(request),
            token_decoder: JwtApiTokenDecoder::new(&key.api),
        }
    }
}

impl<'a> AuthServiceMetadata for ApiServiceMetadata<'a> {
    fn metadata(&self) -> Result<AuthServiceMetadataContent, AuthServiceMetadataError> {
        Ok(AuthServiceMetadataContent {
            nonce: fetch_nonce(&self.nonce_metadata)?,
            token: decode_token(&self.token_metadata, &self.token_decoder)?,
        })
    }
}

pub struct NoAuthorizedServiceMetadata<'a> {
    nonce_metadata: ActixWebAuthNonceMetadata<'a>,
}

impl<'a> NoAuthorizedServiceMetadata<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self {
            nonce_metadata: ActixWebAuthNonceMetadata::new(request),
        }
    }
}

impl<'a> AuthServiceMetadata for NoAuthorizedServiceMetadata<'a> {
    fn metadata(&self) -> Result<AuthServiceMetadataContent, AuthServiceMetadataError> {
        Ok(AuthServiceMetadataContent {
            nonce: fetch_nonce(&self.nonce_metadata)?,
            token: None,
        })
    }
}

fn fetch_nonce(
    nonce_metadata: &impl AuthNonceMetadata,
) -> Result<Option<AuthNonce>, AuthServiceMetadataError> {
    nonce_metadata
        .nonce()
        .map_err(AuthServiceMetadataError::MetadataError)
}
fn decode_token(
    token_metadata: &impl AuthTokenMetadata,
    token_decoder: &impl AuthTokenDecoder,
) -> Result<Option<AuthToken>, AuthServiceMetadataError> {
    match token_metadata
        .token()
        .map_err(AuthServiceMetadataError::MetadataError)?
    {
        None => Ok(None),
        Some(token) => {
            token_decoder
                .decode(&token)
                .map_err(AuthServiceMetadataError::DecodeError)?;
            Ok(Some(token))
        }
    }
}
