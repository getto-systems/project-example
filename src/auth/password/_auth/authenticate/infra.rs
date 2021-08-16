use crate::auth::{
    auth_ticket::_auth::kernel::infra::CheckAuthNonceInfra,
    auth_user::_auth::kernel::infra::AuthUserRepository,
    password::{
        _auth::kernel::infra::AuthUserPasswordMatchInfra,
        _common::authenticate::infra::AuthenticatePasswordFieldsExtract,
    },
};

pub trait AuthenticatePasswordInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type UserRepository: AuthUserRepository;
    type PasswordInfra: AuthUserPasswordMatchInfra;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn user_repository(&self) -> &Self::UserRepository;
    fn password_infra(&self) -> &Self::PasswordInfra;
}

pub trait AuthenticatePasswordRequestDecoder {
    fn decode(self) -> AuthenticatePasswordFieldsExtract;
}
