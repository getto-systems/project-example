pub(in crate::auth) mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::{
    auth_ticket::_auth::kernel::init::CheckAuthNonceStruct,
    auth_user::_auth::kernel::init::user_repository::MysqlAuthUserRepository,
    password::_auth::kernel::init::AuthUserPasswordStruct,
};

use super::infra::AuthenticatePasswordInfra;

pub struct AuthenticatePasswordStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    user_repository: MysqlAuthUserRepository<'a>,
    password_infra: AuthUserPasswordStruct<'a>,
}

impl<'a> AuthenticatePasswordStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, metadata),
            user_repository: MysqlAuthUserRepository::new(&feature.store.mysql),
            password_infra: AuthUserPasswordStruct::new(feature),
        }
    }
}

impl<'a> AuthenticatePasswordInfra for AuthenticatePasswordStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type UserRepository = MysqlAuthUserRepository<'a>;
    type PasswordInfra = AuthUserPasswordStruct<'a>;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
    fn password_infra(&self) -> &Self::PasswordInfra {
        &self.password_infra
    }
}

#[cfg(test)]
pub mod test {
    pub use super::request_decoder::test::StaticAuthenticatePasswordRequestDecoder;

    use super::super::infra::AuthenticatePasswordInfra;
    use crate::auth::{
        auth_ticket::_auth::kernel::init::test::StaticCheckAuthNonceStruct,
        auth_user::_auth::kernel::init::test::MemoryAuthUserRepository,
        password::_auth::kernel::init::test::StaticAuthUserPasswordStruct,
    };

    pub struct StaticAuthenticatePasswordStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub user_repository: MemoryAuthUserRepository<'a>,
        pub password_infra: StaticAuthUserPasswordStruct<'a>,
    }

    impl<'a> AuthenticatePasswordInfra for StaticAuthenticatePasswordStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type UserRepository = MemoryAuthUserRepository<'a>;
        type PasswordInfra = StaticAuthUserPasswordStruct<'a>;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
        fn password_infra(&self) -> &Self::PasswordInfra {
            &self.password_infra
        }
    }
}
