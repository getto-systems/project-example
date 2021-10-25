use crate::auth::ticket::remote::validate::infra::ValidateAuthTokenInfra;

use crate::{
    auth::user::account::remote::search::data::SearchUserAccountBasket,
    z_lib::remote::{
        repository::data::RepositoryError,
        search::data::{SearchSort, SearchSortExtract},
    },
};

pub trait SearchUserAccountInfra {
    type ValidateInfra: ValidateAuthTokenInfra;
    type SearchRepository: SearchUserAccountRepository;

    fn validate_infra(&self) -> &Self::ValidateInfra;
    fn search_repository(&self) -> &Self::SearchRepository;
}

pub struct SearchUserAccountFields {
    offset: u32,
    sort: SearchSort,
    login_id: String,
}

impl SearchUserAccountFields {
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

pub struct SearchUserAccountFieldsExtract {
    pub offset: u32,
    pub sort: SearchSortExtract,
    pub login_id: String,
}
impl Into<SearchUserAccountFields> for SearchUserAccountFieldsExtract {
    fn into(self) -> SearchUserAccountFields {
        SearchUserAccountFields {
            offset: self.offset,
            sort: self.sort.into(),
            login_id: self.login_id,
        }
    }
}

pub trait SearchUserAccountRequestDecoder {
    fn decode(self) -> SearchUserAccountFieldsExtract;
}

#[async_trait::async_trait]
pub trait SearchUserAccountRepository {
    async fn search(
        &self,
        fields: &SearchUserAccountFields,
    ) -> Result<SearchUserAccountBasket, RepositoryError>;
}
