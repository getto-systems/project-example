pub mod request_decoder;
pub mod search_repository;

use tonic::metadata::MetadataMap;

use crate::auth::remote::x_outside_feature::auth::feature::AuthOutsideFeature;

use crate::auth::{
    ticket::remote::validate::init::ApiValidateAuthTokenStruct,
    user::account::remote::search::init::search_repository::MysqlSearchUserAccountRepository,
};

use super::infra::SearchUserAccountInfra;

pub struct SearchUserAccountStruct<'a> {
    validate_infra: ApiValidateAuthTokenStruct<'a>,
    search_repository: MysqlSearchUserAccountRepository<'a>,
}

impl<'a> SearchUserAccountStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
        Self {
            validate_infra: ApiValidateAuthTokenStruct::new(feature, metadata),
            search_repository: MysqlSearchUserAccountRepository::new(&feature.store.mysql),
        }
    }
}

impl<'a> SearchUserAccountInfra for SearchUserAccountStruct<'a> {
    type ValidateInfra = ApiValidateAuthTokenStruct<'a>;
    type SearchRepository = MysqlSearchUserAccountRepository<'a>;

    fn validate_infra(&self) -> &Self::ValidateInfra {
        &self.validate_infra
    }
    fn search_repository(&self) -> &Self::SearchRepository {
        &self.search_repository
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::{
        ticket::remote::validate::init::test::StaticValidateAuthTokenStruct,
        user::account::remote::search::init::search_repository::test::MemorySearchUserAccountRepository,
    };

    use super::super::infra::SearchUserAccountInfra;

    pub struct StaticSearchUserAccountStruct<'a> {
        pub validate_infra: StaticValidateAuthTokenStruct<'a>,
        pub search_repository: MemorySearchUserAccountRepository<'a>,
    }

    impl<'a> SearchUserAccountInfra for StaticSearchUserAccountStruct<'a> {
        type ValidateInfra = StaticValidateAuthTokenStruct<'a>;
        type SearchRepository = MemorySearchUserAccountRepository<'a>;

        fn validate_infra(&self) -> &Self::ValidateInfra {
            &self.validate_infra
        }
        fn search_repository(&self) -> &Self::SearchRepository {
            &self.search_repository
        }
    }
}
