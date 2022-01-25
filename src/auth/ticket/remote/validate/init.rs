pub mod request_decoder;
pub mod token_metadata;

use tonic::metadata::MetadataMap;

use crate::auth::remote::x_outside_feature::auth::feature::AuthOutsideFeature;

use crate::auth::ticket::remote::{
    validate_nonce::init::ValidateAuthNonceStruct,
    kernel::init::token_decoder::{JwtApiTokenDecoder, JwtTicketTokenDecoder},
};
use token_metadata::TonicAuthTokenMetadata;

use super::infra::ValidateAuthTokenInfra;

pub struct TicketValidateAuthTokenStruct<'a> {
    check_nonce_infra: ValidateAuthNonceStruct<'a>,
    token_metadata: TonicAuthTokenMetadata<'a>,
    token_decoder: JwtTicketTokenDecoder<'a>,
}

impl<'a> TicketValidateAuthTokenStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
        Self {
            check_nonce_infra: ValidateAuthNonceStruct::new(feature, metadata),
            token_metadata: TonicAuthTokenMetadata::new(metadata),
            token_decoder: JwtTicketTokenDecoder::new(&feature.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for TicketValidateAuthTokenStruct<'a> {
    type CheckNonceInfra = ValidateAuthNonceStruct<'a>;
    type TokenMetadata = TonicAuthTokenMetadata<'a>;
    type TokenDecoder = JwtTicketTokenDecoder<'a>;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn token_metadata(&self) -> &Self::TokenMetadata {
        &self.token_metadata
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
}

pub struct ApiValidateAuthTokenStruct<'a> {
    check_nonce_infra: ValidateAuthNonceStruct<'a>,
    token_metadata: TonicAuthTokenMetadata<'a>,
    token_decoder: JwtApiTokenDecoder<'a>,
}

impl<'a> ApiValidateAuthTokenStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
        Self {
            check_nonce_infra: ValidateAuthNonceStruct::new(feature, metadata),
            token_metadata: TonicAuthTokenMetadata::new(metadata),
            token_decoder: JwtApiTokenDecoder::new(&feature.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for ApiValidateAuthTokenStruct<'a> {
    type CheckNonceInfra = ValidateAuthNonceStruct<'a>;
    type TokenMetadata = TonicAuthTokenMetadata<'a>;
    type TokenDecoder = JwtApiTokenDecoder<'a>;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn token_metadata(&self) -> &Self::TokenMetadata {
        &self.token_metadata
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::remote::{
        validate_nonce::init::test::StaticValidateAuthNonceStruct,
        kernel::init::{
            token_decoder::test::StaticAuthTokenDecoder,
            token_metadata::test::StaticAuthTokenMetadata,
        },
    };

    use super::super::infra::ValidateAuthTokenInfra;

    pub struct StaticValidateAuthTokenStruct<'a> {
        pub check_nonce_infra: StaticValidateAuthNonceStruct<'a>,
        pub token_metadata: StaticAuthTokenMetadata,
        pub token_decoder: StaticAuthTokenDecoder,
    }

    impl<'a> ValidateAuthTokenInfra for StaticValidateAuthTokenStruct<'a> {
        type CheckNonceInfra = StaticValidateAuthNonceStruct<'a>;
        type TokenMetadata = StaticAuthTokenMetadata;
        type TokenDecoder = StaticAuthTokenDecoder;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn token_metadata(&self) -> &Self::TokenMetadata {
            &self.token_metadata
        }
        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
    }
}
