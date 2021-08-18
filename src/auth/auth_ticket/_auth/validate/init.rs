pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod token_decoder;
pub(in crate::auth) mod token_metadata;

use tonic::metadata::MetadataMap;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_auth::kernel::init::CheckAuthNonceStruct;
use token_decoder::{JwtApiTokenDecoder, JwtAuthTokenDecoder};
use token_metadata::TonicAuthTokenMetadata;

use super::infra::ValidateAuthTokenInfra;

pub struct TicketValidateAuthTokenStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    token_metadata: TonicAuthTokenMetadata<'a>,
    token_decoder: JwtAuthTokenDecoder<'a>,
}

impl<'a> TicketValidateAuthTokenStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, metadata),
            token_metadata: TonicAuthTokenMetadata::new(metadata),
            token_decoder: JwtAuthTokenDecoder::new(&feature.secret.ticket.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for TicketValidateAuthTokenStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type TokenMetadata = TonicAuthTokenMetadata<'a>;
    type TokenDecoder = JwtAuthTokenDecoder<'a>;

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
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    token_metadata: TonicAuthTokenMetadata<'a>,
    token_decoder: JwtApiTokenDecoder<'a>,
}

impl<'a> ApiValidateAuthTokenStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, metadata),
            token_metadata: TonicAuthTokenMetadata::new(metadata),
            token_decoder: JwtApiTokenDecoder::new(&feature.secret.api.decoding_key),
        }
    }
}

impl<'a> ValidateAuthTokenInfra for ApiValidateAuthTokenStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
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
    use super::token_decoder::test::StaticAuthTokenDecoder;
    use super::token_metadata::test::StaticAuthTokenMetadata;
    use crate::auth::auth_ticket::_auth::kernel::init::test::StaticCheckAuthNonceStruct;

    use super::super::infra::ValidateAuthTokenInfra;

    pub struct StaticValidateAuthTokenStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub token_metadata: StaticAuthTokenMetadata,
        pub token_decoder: StaticAuthTokenDecoder,
    }

    impl<'a> ValidateAuthTokenInfra for StaticValidateAuthTokenStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
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
