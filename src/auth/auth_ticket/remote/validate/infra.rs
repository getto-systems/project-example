use crate::auth::auth_ticket::remote::{
    check_nonce::infra::CheckAuthNonceInfra,
    kernel::infra::{AuthTokenDecoder, AuthTokenMetadata},
};

use crate::auth::auth_user::remote::kernel::data::RequireAuthRoles;

pub trait ValidateAuthTokenInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type TokenMetadata: AuthTokenMetadata;
    type TokenDecoder: AuthTokenDecoder;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn token_metadata(&self) -> &Self::TokenMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
}

pub trait ValidateApiTokenRequestDecoder {
    fn decode(self) -> RequireAuthRoles;
}
