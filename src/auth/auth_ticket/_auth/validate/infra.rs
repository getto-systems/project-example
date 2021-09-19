use crate::auth::auth_ticket::_common::kernel::infra::AuthTokenDecoder;
use crate::auth::auth_ticket::{
    _auth::kernel::infra::CheckAuthNonceInfra, _common::kernel::infra::AuthTokenMetadata,
};

use crate::auth::auth_user::_common::kernel::data::RequireAuthRoles;

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
