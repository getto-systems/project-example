use crate::auth::ticket::remote::validate::infra::ValidateAuthTokenInfra;

use crate::{
    auth::user::account::remote::search::data::SearchAuthUserAccountBasket,
    z_lib::remote::{
        repository::data::RepositoryError,
        search::data::{SearchSort, SearchSortExtract},
    },
};

pub trait SearchAuthUserAccountInfra {
    type ValidateInfra: ValidateAuthTokenInfra;
    type SearchRepository: SearchAuthUserAccountRepository;

    fn validate_infra(&self) -> &Self::ValidateInfra;
    fn search_repository(&self) -> &Self::SearchRepository;
}

pub struct SearchAuthUserAccountFields {
    offset: u32,
    sort: SearchSort,
    login_id: String,
}

impl SearchAuthUserAccountFields {
    pub fn offset(&self) -> u32 {
        self.offset
    }
    pub fn sort(&self) -> &SearchSort {
        &self.sort
    }
    pub fn login_id(&self) -> &String {
        &self.login_id
    }
}

pub struct SearchAuthUserAccountFieldsExtract {
    pub offset: u32,
    pub sort: SearchSortExtract,
    pub login_id: String,
}
impl Into<SearchAuthUserAccountFields> for SearchAuthUserAccountFieldsExtract {
    fn into(self) -> SearchAuthUserAccountFields {
        SearchAuthUserAccountFields {
            offset: self.offset,
            sort: self.sort.into(),
            login_id: self.login_id,
        }
    }
}

pub trait SearchAuthUserAccountRequestDecoder {
    fn decode(self) -> SearchAuthUserAccountFieldsExtract;
}

#[async_trait::async_trait]
pub trait SearchAuthUserAccountRepository {
    async fn search(
        &self,
        fields: &SearchAuthUserAccountFields,
    ) -> Result<SearchAuthUserAccountBasket, RepositoryError>;
}
