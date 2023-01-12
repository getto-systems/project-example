mod request;

use crate::x_outside_feature::{auth::feature::AuthAppFeature, data::RequestId};

use crate::auth::{
    ticket::authorize::init::ActiveAuthorizeInfra,
    user::kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
};

use crate::auth::user::login_id::change::action::{
    OverwriteLoginIdAction, OverwriteLoginIdMaterial,
};

pub struct ActiveOverwriteLoginIdMaterial<'a> {
    authorize: ActiveAuthorizeInfra<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> ActiveOverwriteLoginIdMaterial<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        request_id: RequestId,
    ) -> OverwriteLoginIdAction<Self> {
        OverwriteLoginIdAction::with_material(Self {
            authorize: ActiveAuthorizeInfra::from_auth(feature, request_id),
            user_repository: DynamoDbAuthUserRepository::new(&feature.store),
        })
    }
}

impl<'a> OverwriteLoginIdMaterial for ActiveOverwriteLoginIdMaterial<'a> {
    type Authorize = ActiveAuthorizeInfra<'a>;

    type LoginIdRepository = DynamoDbAuthUserRepository<'a>;

    fn authorize(&self) -> &Self::Authorize {
        &self.authorize
    }
    fn login_id_repository(&self) -> &Self::LoginIdRepository {
        &self.user_repository
    }
}

#[cfg(test)]
pub mod test {
    pub use super::request::test::*;

    use crate::auth::{
        ticket::authorize::init::test::StaticAuthorizeInfra,
        user::kernel::init::user_repository::memory::MemoryAuthUserRepository,
    };

    use crate::auth::user::login_id::change::action::OverwriteLoginIdMaterial;

    pub struct StaticOverwriteLoginIdMaterial<'a> {
        pub authorize: StaticAuthorizeInfra,
        pub login_id_repository: MemoryAuthUserRepository<'a>,
    }

    impl<'a> OverwriteLoginIdMaterial for StaticOverwriteLoginIdMaterial<'a> {
        type Authorize = StaticAuthorizeInfra;

        type LoginIdRepository = MemoryAuthUserRepository<'a>;

        fn authorize(&self) -> &Self::Authorize {
            &self.authorize
        }
        fn login_id_repository(&self) -> &Self::LoginIdRepository {
            &self.login_id_repository
        }
    }
}
