use crate::auth::{
    auth_ticket::_auth::validate::infra::ValidateAuthTokenInfra,
    password::{
        _auth::kernel::infra::{
            AuthUserPasswordHasher, AuthUserPasswordMatcher, ChangePasswordRepository,
            PlainPassword,
        },
        _common::change::infra::ChangePasswordFieldsExtract,
    },
};

pub trait ChangePasswordInfra {
    type ValidateInfra: ValidateAuthTokenInfra;
    type PasswordRepository: ChangePasswordRepository;
    type PasswordMatcher: AuthUserPasswordMatcher;
    type PasswordHasher: AuthUserPasswordHasher;

    fn validate_infra(&self) -> &Self::ValidateInfra;
    fn password_repository(&self) -> &Self::PasswordRepository;
    fn password_matcher(&self, plain_password: PlainPassword) -> Self::PasswordMatcher {
        Self::PasswordMatcher::new(plain_password)
    }
    fn password_hasher(&self, plain_password: PlainPassword) -> Self::PasswordHasher {
        Self::PasswordHasher::new(plain_password)
    }
}

pub trait ChangePasswordRequestDecoder {
    fn decode(self) -> ChangePasswordFieldsExtract;
}
