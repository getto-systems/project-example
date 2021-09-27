use crate::auth::auth_ticket::remote::kernel::infra::{AuthMetadata, AuthTokenDecoder};

pub trait ValidateAuthMetadataInfra {
    type AuthMetadata: AuthMetadata;
    type TokenDecoder: AuthTokenDecoder;

    fn auth_metadata(&self) -> &Self::AuthMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
}
