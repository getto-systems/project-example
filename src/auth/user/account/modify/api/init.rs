use crate::x_outside_feature::{auth::feature::AuthAppFeature, data::RequestId};

use crate::auth::{
    ticket::authorize::init::ActiveAuthorizeInfra,
    user::kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
};

use crate::auth::user::account::modify::action::{
    ModifyAuthUserAccountAction, ModifyAuthUserAccountMaterial,
};

pub struct ActiveModifyAuthUserAccountMaterial<'a> {
    authorize: ActiveAuthorizeInfra<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> ActiveModifyAuthUserAccountMaterial<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        request_id: RequestId,
    ) -> ModifyAuthUserAccountAction<Self> {
        ModifyAuthUserAccountAction::with_material(Self {
            authorize: ActiveAuthorizeInfra::from_auth(feature, request_id),
            user_repository: DynamoDbAuthUserRepository::new(&feature.store),
        })
    }
}

impl<'a> ModifyAuthUserAccountMaterial for ActiveModifyAuthUserAccountMaterial<'a> {
    type Authorize = ActiveAuthorizeInfra<'a>;

    type UserRepository = DynamoDbAuthUserRepository<'a>;

    fn authorize(&self) -> &Self::Authorize {
        &self.authorize
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::{
        ticket::authorize::init::test::StaticAuthorizeInfra,
        user::kernel::init::user_repository::memory::MemoryAuthUserRepository,
    };

    use crate::auth::user::account::modify::action::ModifyAuthUserAccountMaterial;

    use crate::auth::user::account::modify::infra::{
        ModifyAuthUserAccountFields, ModifyAuthUserAccountFieldsExtract,
    };

    use crate::auth::user::account::modify::data::ValidateModifyAuthUserAccountFieldsError;

    pub enum StaticModifyAuthUserAccountFields {
        Valid(ModifyAuthUserAccountFields),
        Invalid(ValidateModifyAuthUserAccountFieldsError),
    }

    impl ModifyAuthUserAccountFieldsExtract for StaticModifyAuthUserAccountFields {
        fn convert(
            self,
        ) -> Result<ModifyAuthUserAccountFields, ValidateModifyAuthUserAccountFieldsError> {
            match self {
                Self::Valid(fields) => Ok(fields),
                Self::Invalid(err) => Err(err),
            }
        }
    }

    pub struct StaticModifyAuthUserAccountMaterial<'a> {
        pub authorize: StaticAuthorizeInfra,
        pub user_repository: MemoryAuthUserRepository<'a>,
    }

    impl<'a> ModifyAuthUserAccountMaterial for StaticModifyAuthUserAccountMaterial<'a> {
        type Authorize = StaticAuthorizeInfra;

        type UserRepository = MemoryAuthUserRepository<'a>;

        fn authorize(&self) -> &Self::Authorize {
            &self.authorize
        }
        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
    }
}
