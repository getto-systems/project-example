use crate::{
    auth::user::account::search::data::SearchAuthUserAccountBasket,
    z_lib::{
        repository::data::RepositoryError,
        search::data::{SearchSort, SearchSortExtract},
    },
};

pub struct SearchAuthUserAccountFields {
    offset: i32,
    sort: SearchSort,
    login_id: String,
}

impl SearchAuthUserAccountFields {
    pub fn offset(&self) -> i32 {
        self.offset
    }
    pub fn sort(&self) -> &SearchSort {
        &self.sort
    }
    pub fn login_id(&self) -> &str {
        &self.login_id
    }
}

pub struct SearchAuthUserAccountFieldsExtract {
    pub offset: i32,
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
