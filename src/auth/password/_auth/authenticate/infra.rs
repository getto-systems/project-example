use crate::auth::{
    auth_ticket::_auth::kernel::infra::CheckAuthNonceInfra,
    auth_user::_auth::kernel::infra::AuthUserRepository,
    password::{
        _auth::kernel::infra::{
            AuthUserPasswordMatcher, AuthUserPasswordRepository, PlainPassword,
        },
        _common::authenticate::infra::AuthenticatePasswordFieldsExtract,
    },
};

pub trait AuthenticatePasswordInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type UserRepository: AuthUserRepository;
    type PasswordRepository: AuthUserPasswordRepository;
    type PasswordMatcher: AuthUserPasswordMatcher;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn user_repository(&self) -> &Self::UserRepository;
    fn password_repository(&self) -> &Self::PasswordRepository;
    fn password_matcher(&self, plain_password: PlainPassword) -> Self::PasswordMatcher {
        Self::PasswordMatcher::new(plain_password)
    }
}

pub trait AuthenticatePasswordRequestDecoder {
    fn decode(self) -> AuthenticatePasswordFieldsExtract;
}
