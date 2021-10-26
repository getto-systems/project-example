pub mod request_decoder;
pub mod search_repository;

use tonic::metadata::MetadataMap;

use crate::auth::remote::x_outside_feature::auth::feature::AuthOutsideFeature;

use crate::auth::{
    ticket::remote::validate::init::ApiValidateAuthTokenStruct,
    user::account::remote::search::init::search_repository::MysqlSearchAuthUserAccountRepository,
};

use super::infra::SearchAuthUserAccountInfra;

pub struct SearchAuthUserAccountStruct<'a> {
    validate_infra: ApiValidateAuthTokenStruct<'a>,
    search_repository: MysqlSearchAuthUserAccountRepository<'a>,
}

impl<'a> SearchAuthUserAccountStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
        Self {
            validate_infra: ApiValidateAuthTokenStruct::new(feature, metadata),
            search_repository: MysqlSearchAuthUserAccountRepository::new(&feature.store.mysql),
        }
    }
}

impl<'a> SearchAuthUserAccountInfra for SearchAuthUserAccountStruct<'a> {
    type ValidateInfra = ApiValidateAuthTokenStruct<'a>;
    type SearchRepository = MysqlSearchAuthUserAccountRepository<'a>;

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
        user::account::remote::search::init::search_repository::test::MemorySearchAuthUserAccountRepository,
    };

    use super::super::infra::SearchAuthUserAccountInfra;

    pub struct StaticSearchAuthUserAccountStruct<'a> {
        pub validate_infra: StaticValidateAuthTokenStruct<'a>,
        pub search_repository: MemorySearchAuthUserAccountRepository<'a>,
    }

    impl<'a> SearchAuthUserAccountInfra for StaticSearchAuthUserAccountStruct<'a> {
        type ValidateInfra = StaticValidateAuthTokenStruct<'a>;
        type SearchRepository = MemorySearchAuthUserAccountRepository<'a>;

        fn validate_infra(&self) -> &Self::ValidateInfra {
            &self.validate_infra
        }
        fn search_repository(&self) -> &Self::SearchRepository {
            &self.search_repository
        }
    }
}
