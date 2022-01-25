use crate::auth::ticket::remote::{
    validate_nonce::infra::ValidateAuthNonceInfra,
    kernel::infra::{AuthTokenDecoder, AuthTokenMetadata},
};

use crate::auth::user::remote::kernel::data::RequireAuthRoles;

pub trait ValidateAuthTokenInfra {
    type CheckNonceInfra: ValidateAuthNonceInfra;
    type TokenMetadata: AuthTokenMetadata;
    type TokenDecoder: AuthTokenDecoder;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn token_metadata(&self) -> &Self::TokenMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
}

pub trait ValidateApiTokenRequestDecoder {
    fn decode(self) -> RequireAuthRoles;
}
