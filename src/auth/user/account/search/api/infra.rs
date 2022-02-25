use crate::{
    auth::user::account::search::data::SearchAuthUserAccountBasket,
    z_lib::{
        repository::data::RepositoryError,
        search::data::{SearchSort, SearchSortExtract},
    },
};

pub struct SearchAuthUserAccountFilter {
    offset: i32,
    sort: SearchSort,
    login_id: Option<String>,
}

impl SearchAuthUserAccountFilter {
    pub fn offset(&self) -> i32 {
        self.offset
    }
    pub fn sort(&self) -> &SearchSort {
        &self.sort
    }
    pub fn login_id(&self) -> &Option<String> {
        &self.login_id
    }
}

pub struct SearchAuthUserAccountFilterExtract {
    pub offset: i32,
    pub sort: SearchSortExtract,
    pub login_id: Option<String>,
}
impl Into<SearchAuthUserAccountFilter> for SearchAuthUserAccountFilterExtract {
    fn into(self) -> SearchAuthUserAccountFilter {
        SearchAuthUserAccountFilter {
            offset: self.offset,
            sort: self.sort.into(),
            login_id: self.login_id,
        }
    }
}

pub trait SearchAuthUserAccountRequestDecoder {
    fn decode(self) -> SearchAuthUserAccountFilterExtract;
}

#[async_trait::async_trait]
pub trait SearchAuthUserAccountRepository {
    async fn search(
        &self,
        fields: &SearchAuthUserAccountFilter,
    ) -> Result<SearchAuthUserAccountBasket, RepositoryError>;
}
