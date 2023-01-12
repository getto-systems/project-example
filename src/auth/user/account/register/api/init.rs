mod user_id_generator;

use crate::auth::ticket::authorize::init::ActiveAuthorizeInfra;

use crate::x_outside_feature::{auth::feature::AuthAppFeature, data::RequestId};

use crate::auth::user::{
    account::register::init::user_id_generator::UuidAuthUserIdGenerator,
    kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
};

use crate::auth::user::account::register::action::{
    RegisterAuthUserAccountAction, RegisterAuthUserAccountMaterial,
};

pub struct ActiveRegisterAuthUserAccountMaterial<'a> {
    authorize: ActiveAuthorizeInfra<'a>,
    user_id_generator: UuidAuthUserIdGenerator,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> ActiveRegisterAuthUserAccountMaterial<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        request_id: RequestId,
    ) -> RegisterAuthUserAccountAction<Self> {
        RegisterAuthUserAccountAction::with_material(Self {
            authorize: ActiveAuthorizeInfra::from_auth(feature, request_id),
            user_id_generator: UuidAuthUserIdGenerator::new(),
            user_repository: DynamoDbAuthUserRepository::new(&feature.store),
        })
    }
}

impl<'a> RegisterAuthUserAccountMaterial for ActiveRegisterAuthUserAccountMaterial<'a> {
    type Authorize = ActiveAuthorizeInfra<'a>;

    type UserIdGenerator = UuidAuthUserIdGenerator;
    type UserRepository = DynamoDbAuthUserRepository<'a>;

    fn authorize(&self) -> &Self::Authorize {
        &self.authorize
    }

    fn user_id_generator(&self) -> &Self::UserIdGenerator {
        &self.user_id_generator
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}

#[cfg(test)]
pub mod test {
    pub use crate::auth::user::account::register::init::user_id_generator::test::StaticAuthUserIdGenerator;

    use crate::auth::{
        ticket::authorize::init::test::StaticAuthorizeInfra,
        user::kernel::init::user_repository::memory::MemoryAuthUserRepository,
    };

    use crate::auth::user::account::register::action::RegisterAuthUserAccountMaterial;

    use crate::auth::user::account::register::infra::RegisterAuthUserAccountFieldsExtract;

    use crate::auth::user::account::kernel::data::{AuthUserAccount, ValidateAuthUserAccountError};

    pub enum StaticRegisterAuthUserAccountFields {
        Valid(AuthUserAccount),
        Invalid(ValidateAuthUserAccountError),
    }

    impl RegisterAuthUserAccountFieldsExtract for StaticRegisterAuthUserAccountFields {
        fn convert(self) -> Result<AuthUserAccount, ValidateAuthUserAccountError> {
            match self {
                Self::Valid(fields) => Ok(fields),
                Self::Invalid(err) => Err(err),
            }
        }
    }

    pub struct StaticRegisterAuthUserAccountMaterial<'a> {
        pub authorize: StaticAuthorizeInfra,
        pub user_id_generator: StaticAuthUserIdGenerator,
        pub user_repository: MemoryAuthUserRepository<'a>,
    }

    impl<'a> RegisterAuthUserAccountMaterial for StaticRegisterAuthUserAccountMaterial<'a> {
        type Authorize = StaticAuthorizeInfra;

        type UserIdGenerator = StaticAuthUserIdGenerator;
        type UserRepository = MemoryAuthUserRepository<'a>;

        fn authorize(&self) -> &Self::Authorize {
            &self.authorize
        }

        fn user_id_generator(&self) -> &Self::UserIdGenerator {
            &self.user_id_generator
        }
        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
    }
}
