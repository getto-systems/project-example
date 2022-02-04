pub mod auth_metadata;
pub mod nonce_metadata;
pub mod nonce_repository;
pub mod request_decoder;
pub mod token_decoder;
pub mod token_metadata;
pub mod validate_service;

use actix_web::HttpRequest;
use tonic::metadata::MetadataMap;

use crate::auth::ticket::y_protobuf::service::ValidateApiTokenRequestPb;

use crate::{
    auth::x_outside_feature::remote::{
        auth::feature::AuthOutsideFeature,
        common::feature::{AuthOutsideDecodingKey, AuthOutsideService},
    },
    x_outside_feature::remote::auth::feature::AuthAppFeature,
};

use crate::auth::ticket::{
    kernel::remote::init::clock::ChronoAuthClock,
    validate::init::{
        auth_metadata::{ApiAuthMetadata, NoAuthMetadata, TicketAuthMetadata, TonicAuthMetadata},
        nonce_metadata::TonicAuthNonceMetadata,
        nonce_repository::DynamoDbAuthNonceRepository,
        request_decoder::PbValidateApiTokenRequestDecoder,
        token_decoder::{JwtApiTokenDecoder, JwtTicketTokenDecoder, NoopTokenDecoder},
        token_metadata::TonicAuthTokenMetadata,
        validate_service::TonicValidateService,
    },
};

use crate::auth::ticket::validate::action::ValidateApiTokenAction;

use crate::auth::ticket::validate::method::{
    AuthNonceConfig, CheckPermissionInfra, ValidateAuthMetadataInfra, ValidateAuthNonceInfra,
    ValidateAuthTokenInfra,
};

pub struct ValidateApiTokenStruct;

impl ValidateApiTokenStruct {
    pub fn action<'a>(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: ValidateApiTokenRequestPb,
    ) -> ValidateApiTokenAction<PbValidateApiTokenRequestDecoder, ApiValidateAuthTokenStruct<'a>>
    {
        ValidateApiTokenAction::with_material(
            PbValidateApiTokenRequestDecoder::new(request),
            ApiValidateAuthTokenStruct::new(&feature.auth, metadata),
        )
    }
}

pub struct TicketValidateAuthTokenStruct<'a> {
    validate_nonce: ValidateAuthNonceStruct<'a>,
    token_metadata: TonicAuthTokenMetadata<'a>,
    token_decoder: JwtTicketTokenDecoder<'a>,
}

impl<'a> TicketValidateAuthTokenStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
        Self {
            validate_nonce: ValidateAuthNonceStruct::new(feature, metadata),
            token_metadata: TonicAuthTokenMetadata::new(metadata),
            token_decoder: JwtTicketTokenDecoder::new(&feature.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for TicketValidateAuthTokenStruct<'a> {
    type ValidateNonce = ValidateAuthNonceStruct<'a>;
    type TokenMetadata = TonicAuthTokenMetadata<'a>;
    type TokenDecoder = JwtTicketTokenDecoder<'a>;

    fn validate_nonce(&self) -> &Self::ValidateNonce {
        &self.validate_nonce
    }
    fn token_metadata(&self) -> &Self::TokenMetadata {
        &self.token_metadata
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
}

pub struct ApiValidateAuthTokenStruct<'a> {
    validate_nonce: ValidateAuthNonceStruct<'a>,
    token_metadata: TonicAuthTokenMetadata<'a>,
    token_decoder: JwtApiTokenDecoder<'a>,
}

impl<'a> ApiValidateAuthTokenStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
        Self {
            validate_nonce: ValidateAuthNonceStruct::new(feature, metadata),
            token_metadata: TonicAuthTokenMetadata::new(metadata),
            token_decoder: JwtApiTokenDecoder::new(&feature.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for ApiValidateAuthTokenStruct<'a> {
    type ValidateNonce = ValidateAuthNonceStruct<'a>;
    type TokenMetadata = TonicAuthTokenMetadata<'a>;
    type TokenDecoder = JwtApiTokenDecoder<'a>;

    fn validate_nonce(&self) -> &Self::ValidateNonce {
        &self.validate_nonce
    }
    fn token_metadata(&self) -> &Self::TokenMetadata {
        &self.token_metadata
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
}

pub struct CheckPermissionStruct<'a> {
    auth_metadata: TonicAuthMetadata<'a>,
    token_decoder: NoopTokenDecoder,
    validate_service: TonicValidateService<'a>,
}

impl<'a> CheckPermissionStruct<'a> {
    pub fn new(
        service: &'a AuthOutsideService,
        request_id: &'a str,
        metadata: &'a MetadataMap,
    ) -> Self {
        Self {
            auth_metadata: TonicAuthMetadata::new(metadata),
            token_decoder: NoopTokenDecoder,
            validate_service: TonicValidateService::new(&service, request_id),
        }
    }
}

impl<'a> CheckPermissionInfra for CheckPermissionStruct<'a> {
    type AuthMetadata = TonicAuthMetadata<'a>;
    type TokenDecoder = NoopTokenDecoder;
    type ValidateService = TonicValidateService<'a>;

    fn auth_metadata(&self) -> &Self::AuthMetadata {
        &self.auth_metadata
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
    fn validate_service(&self) -> &Self::ValidateService {
        &self.validate_service
    }
}

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

pub struct ValidateAuthNonceStruct<'a> {
    config: AuthNonceConfig,
    clock: ChronoAuthClock,
    nonce_metadata: TonicAuthNonceMetadata<'a>,
    nonce_repository: DynamoDbAuthNonceRepository<'a>,
}

impl<'a> ValidateAuthNonceInfra for ValidateAuthNonceStruct<'a> {
    type Clock = ChronoAuthClock;
    type NonceMetadata = TonicAuthNonceMetadata<'a>;
    type NonceRepository = DynamoDbAuthNonceRepository<'a>;

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn nonce_metadata(&self) -> &Self::NonceMetadata {
        &self.nonce_metadata
    }
    fn nonce_repository(&self) -> &Self::NonceRepository {
        &self.nonce_repository
    }
    fn config(&self) -> &AuthNonceConfig {
        &self.config
    }
}

impl<'a> ValidateAuthNonceStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
        Self {
            config: AuthNonceConfig {
                nonce_expires: feature.config.ticket_expires,
            },
            clock: ChronoAuthClock::new(),
            nonce_metadata: TonicAuthNonceMetadata::new(metadata),
            nonce_repository: DynamoDbAuthNonceRepository::new(
                &feature.store.dynamodb,
                feature.store.nonce_table_name,
            ),
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::{
        kernel::remote::init::clock::test::StaticChronoAuthClock,
        validate::init::{
            auth_metadata::test::StaticAuthMetadata, nonce_metadata::test::StaticAuthNonceMetadata,
            nonce_repository::test::MemoryAuthNonceRepository,
            token_decoder::test::StaticAuthTokenDecoder,
            token_metadata::test::StaticAuthTokenMetadata,
            validate_service::test::StaticValidateService,
        },
    };

    use crate::auth::ticket::validate::method::{
        AuthNonceConfig, CheckPermissionInfra, ValidateAuthNonceInfra, ValidateAuthTokenInfra,
    };

    pub struct StaticValidateAuthTokenStruct<'a> {
        pub validate_nonce: StaticValidateAuthNonceStruct<'a>,
        pub token_metadata: StaticAuthTokenMetadata,
        pub token_decoder: StaticAuthTokenDecoder,
    }

    impl<'a> ValidateAuthTokenInfra for StaticValidateAuthTokenStruct<'a> {
        type ValidateNonce = StaticValidateAuthNonceStruct<'a>;
        type TokenMetadata = StaticAuthTokenMetadata;
        type TokenDecoder = StaticAuthTokenDecoder;

        fn validate_nonce(&self) -> &Self::ValidateNonce {
            &self.validate_nonce
        }
        fn token_metadata(&self) -> &Self::TokenMetadata {
            &self.token_metadata
        }
        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
    }

    pub struct StaticValidateApiTokenStruct {
        pub auth_metadata: StaticAuthMetadata,
        pub token_decoder: StaticAuthTokenDecoder,
        pub validate_service: StaticValidateService,
    }

    impl CheckPermissionInfra for StaticValidateApiTokenStruct {
        type AuthMetadata = StaticAuthMetadata;
        type TokenDecoder = StaticAuthTokenDecoder;
        type ValidateService = StaticValidateService;

        fn auth_metadata(&self) -> &Self::AuthMetadata {
            &self.auth_metadata
        }
        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
        fn validate_service(&self) -> &Self::ValidateService {
            &self.validate_service
        }
    }

    pub struct StaticValidateAuthNonceStruct<'a> {
        pub config: AuthNonceConfig,
        pub clock: StaticChronoAuthClock,
        pub nonce_metadata: StaticAuthNonceMetadata,
        pub nonce_repository: MemoryAuthNonceRepository<'a>,
    }

    impl<'a> ValidateAuthNonceInfra for StaticValidateAuthNonceStruct<'a> {
        type Clock = StaticChronoAuthClock;
        type NonceMetadata = StaticAuthNonceMetadata;
        type NonceRepository = MemoryAuthNonceRepository<'a>;

        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn nonce_metadata(&self) -> &Self::NonceMetadata {
            &self.nonce_metadata
        }
        fn nonce_repository(&self) -> &Self::NonceRepository {
            &self.nonce_repository
        }
        fn config(&self) -> &AuthNonceConfig {
            &self.config
        }
    }
}
