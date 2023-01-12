use crate::x_outside_feature::{auth::feature::AuthAppFeature, data::RequestId};

use crate::auth::{
    ticket::authorize::init::ActiveAuthorizeInfra,
    user::kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
};

use crate::auth::user::account::search::action::{
    SearchAuthUserAccountAction, SearchAuthUserAccountMaterial,
};

pub struct ActiveSearchAuthUserAccountMaterial<'a> {
    authorize: ActiveAuthorizeInfra<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> ActiveSearchAuthUserAccountMaterial<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        request_id: RequestId,
    ) -> SearchAuthUserAccountAction<Self> {
        SearchAuthUserAccountAction::with_material(Self {
            authorize: ActiveAuthorizeInfra::from_auth(feature, request_id),
            user_repository: DynamoDbAuthUserRepository::new(&feature.store),
        })
    }
}

impl<'a> SearchAuthUserAccountMaterial for ActiveSearchAuthUserAccountMaterial<'a> {
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

    use crate::auth::user::account::search::action::SearchAuthUserAccountMaterial;

    use crate::auth::user::account::search::infra::SearchAuthUserAccountFilterExtract;

    use crate::auth::user::account::search::data::SearchAuthUserAccountFilter;

    pub enum StaticSearchAuthUserAccountFilter {
        Valid(SearchAuthUserAccountFilter),
    }

    impl SearchAuthUserAccountFilterExtract for StaticSearchAuthUserAccountFilter {
        fn convert(self) -> SearchAuthUserAccountFilter {
            match self {
                Self::Valid(fields) => fields,
            }
        }
    }

    pub struct StaticSearchAuthUserAccountMaterial<'a> {
        pub authorize: StaticAuthorizeInfra,
        pub user_repository: MemoryAuthUserRepository<'a>,
    }

    impl<'a> SearchAuthUserAccountMaterial for StaticSearchAuthUserAccountMaterial<'a> {
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
