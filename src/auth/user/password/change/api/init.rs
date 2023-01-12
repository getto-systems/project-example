use crate::x_outside_feature::{auth::feature::AuthAppFeature, data::RequestId};

use crate::auth::{
    ticket::authorize::init::ActiveAuthorizeInfra,
    user::{
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
        password::kernel::init::{
            password_hasher::Argon2PasswordHasher, password_matcher::Argon2PasswordMatcher,
        },
    },
};

use crate::auth::user::password::change::action::{
    ChangePasswordAction, ChangePasswordMaterial, OverwritePasswordAction,
    OverwritePasswordMaterial,
};

pub struct ActiveChangePasswordMaterial<'a> {
    authorize: ActiveAuthorizeInfra<'a>,
    password_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> ActiveChangePasswordMaterial<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        request_id: RequestId,
    ) -> ChangePasswordAction<Self> {
        ChangePasswordAction::with_material(Self {
            authorize: ActiveAuthorizeInfra::from_auth(feature, request_id),
            password_repository: DynamoDbAuthUserRepository::new(&feature.store),
        })
    }
}

impl<'a> ChangePasswordMaterial for ActiveChangePasswordMaterial<'a> {
    type Authorize = ActiveAuthorizeInfra<'a>;

    type PasswordRepository = DynamoDbAuthUserRepository<'a>;
    type PasswordMatcher = Argon2PasswordMatcher;
    type PasswordHasher = Argon2PasswordHasher;

    fn authorize(&self) -> &Self::Authorize {
        &self.authorize
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
}

pub struct ActiveOverwritePasswordMaterial<'a> {
    authorize: ActiveAuthorizeInfra<'a>,
    password_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> ActiveOverwritePasswordMaterial<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        request_id: RequestId,
    ) -> OverwritePasswordAction<Self> {
        OverwritePasswordAction::with_material(Self {
            authorize: ActiveAuthorizeInfra::from_auth(feature, request_id),
            password_repository: DynamoDbAuthUserRepository::new(&feature.store),
        })
    }
}

impl<'a> OverwritePasswordMaterial for ActiveOverwritePasswordMaterial<'a> {
    type Authorize = ActiveAuthorizeInfra<'a>;

    type PasswordRepository = DynamoDbAuthUserRepository<'a>;
    type PasswordHasher = Argon2PasswordHasher;

    fn authorize(&self) -> &Self::Authorize {
        &self.authorize
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::{
        ticket::authorize::init::test::StaticAuthorizeInfra,
        user::{
            kernel::init::user_repository::memory::MemoryAuthUserRepository,
            password::kernel::init::{
                password_hasher::test::PlainPasswordHasher,
                password_matcher::test::PlainPasswordMatcher,
            },
        },
    };

    use crate::auth::user::password::change::action::{
        ChangePasswordMaterial, OverwritePasswordMaterial,
    };

    use crate::auth::user::password::change::infra::{
        ChangePasswordFields, ChangePasswordFieldsExtract, OverwritePasswordFields,
        OverwritePasswordFieldsExtract,
    };

    use crate::auth::user::password::change::data::{
        ValidateChangePasswordFieldsError, ValidateOverwritePasswordFieldsError,
    };

    pub enum StaticChangePasswordFields {
        Valid(ChangePasswordFields),
        Invalid(ValidateChangePasswordFieldsError),
    }

    impl ChangePasswordFieldsExtract for StaticChangePasswordFields {
        fn convert(self) -> Result<ChangePasswordFields, ValidateChangePasswordFieldsError> {
            match self {
                Self::Valid(fields) => Ok(fields),
                Self::Invalid(err) => Err(err),
            }
        }
    }

    pub enum StaticOverwritePasswordFields {
        Valid(OverwritePasswordFields),
        Invalid(ValidateOverwritePasswordFieldsError),
    }

    impl OverwritePasswordFieldsExtract for StaticOverwritePasswordFields {
        fn convert(self) -> Result<OverwritePasswordFields, ValidateOverwritePasswordFieldsError> {
            match self {
                Self::Valid(fields) => Ok(fields),
                Self::Invalid(err) => Err(err),
            }
        }
    }

    pub struct StaticChangePasswordMaterial<'a> {
        pub authorize: StaticAuthorizeInfra,
        pub password_repository: MemoryAuthUserRepository<'a>,
    }

    impl<'a> ChangePasswordMaterial for StaticChangePasswordMaterial<'a> {
        type Authorize = StaticAuthorizeInfra;

        type PasswordRepository = MemoryAuthUserRepository<'a>;
        type PasswordMatcher = PlainPasswordMatcher;
        type PasswordHasher = PlainPasswordHasher;

        fn authorize(&self) -> &Self::Authorize {
            &self.authorize
        }
        fn password_repository(&self) -> &Self::PasswordRepository {
            &self.password_repository
        }
    }

    pub struct StaticOverwritePasswordMaterial<'a> {
        pub authorize: StaticAuthorizeInfra,
        pub password_repository: MemoryAuthUserRepository<'a>,
    }

    impl<'a> OverwritePasswordMaterial for StaticOverwritePasswordMaterial<'a> {
        type Authorize = StaticAuthorizeInfra;

        type PasswordRepository = MemoryAuthUserRepository<'a>;
        type PasswordHasher = PlainPasswordHasher;

        fn authorize(&self) -> &Self::Authorize {
            &self.authorize
        }
        fn password_repository(&self) -> &Self::PasswordRepository {
            &self.password_repository
        }
    }
}
