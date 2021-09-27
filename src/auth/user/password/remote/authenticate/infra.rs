use crate::auth::{
    ticket::remote::check_nonce::infra::CheckAuthNonceInfra,
    user::{
        password::remote::{
            kernel::infra::{AuthUserPasswordMatcher, PlainPassword, VerifyPasswordRepository},
            proxy_authenticate::infra::AuthenticatePasswordFieldsExtract,
        },
        remote::kernel::infra::AuthUserRepository,
    },
};

pub trait AuthenticatePasswordInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type UserRepository: AuthUserRepository;
    type PasswordRepository: VerifyPasswordRepository;
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
