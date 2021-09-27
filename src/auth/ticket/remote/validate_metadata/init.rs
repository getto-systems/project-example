use actix_web::HttpRequest;

use crate::auth::remote::x_outside_feature::common::feature::AuthOutsideDecodingKey;

use crate::auth::ticket::remote::kernel::init::{
    auth_metadata::{ApiAuthMetadata, NoAuthMetadata, TicketAuthMetadata},
    token_decoder::{JwtApiTokenDecoder, JwtTicketTokenDecoder, NoopTokenDecoder},
};

use crate::auth::ticket::remote::validate_metadata::infra::ValidateAuthMetadataInfra;

pub struct ValidateTicketMetadataStruct<'a> {
    auth_metadata: TicketAuthMetadata<'a>,
    token_decoder: JwtTicketTokenDecoder<'a>,
}

impl<'a> ValidateTicketMetadataStruct<'a> {
    pub fn new(decoding_key: &'a AuthOutsideDecodingKey, request: &'a HttpRequest) -> Self {
        Self {
            auth_metadata: TicketAuthMetadata::new(request),
            token_decoder: JwtTicketTokenDecoder::new(&decoding_key),
        }
    }
}

#[async_trait::async_trait]
impl<'a> ValidateAuthMetadataInfra for ValidateTicketMetadataStruct<'a> {
    type AuthMetadata = TicketAuthMetadata<'a>;
    type TokenDecoder = JwtTicketTokenDecoder<'a>;

    fn auth_metadata(&self) -> &Self::AuthMetadata {
        &self.auth_metadata
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
}

pub struct ValidateApiMetadataStruct<'a> {
    auth_metadata: ApiAuthMetadata<'a>,
    token_decoder: JwtApiTokenDecoder<'a>,
}

impl<'a> ValidateApiMetadataStruct<'a> {
    pub fn new(decoding_key: &'a AuthOutsideDecodingKey, request: &'a HttpRequest) -> Self {
        Self {
            auth_metadata: ApiAuthMetadata::new(request),
            token_decoder: JwtApiTokenDecoder::new(&decoding_key),
        }
    }
}

#[async_trait::async_trait]
impl<'a> ValidateAuthMetadataInfra for ValidateApiMetadataStruct<'a> {
    type AuthMetadata = ApiAuthMetadata<'a>;
    type TokenDecoder = JwtApiTokenDecoder<'a>;

    fn auth_metadata(&self) -> &Self::AuthMetadata {
        &self.auth_metadata
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
}

pub struct NoValidateMetadataStruct<'a> {
    auth_metadata: NoAuthMetadata<'a>,
    token_decoder: NoopTokenDecoder,
}

impl<'a> NoValidateMetadataStruct<'a> {
    pub fn new(request: &'a HttpRequest) -> Self {
        Self {
            auth_metadata: NoAuthMetadata::new(request),
            token_decoder: NoopTokenDecoder,
        }
    }
}

#[async_trait::async_trait]
impl<'a> ValidateAuthMetadataInfra for NoValidateMetadataStruct<'a> {
    type AuthMetadata = NoAuthMetadata<'a>;
    type TokenDecoder = NoopTokenDecoder;

    fn auth_metadata(&self) -> &Self::AuthMetadata {
        &self.auth_metadata
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
}
