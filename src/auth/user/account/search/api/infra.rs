use crate::{
    auth::user::account::search::data::{AuthUserAccountSearch, SearchAuthUserAccountFilter},
    common::api::repository::data::RepositoryError,
};

pub trait SearchAuthUserAccountFilterExtract {
    fn convert(self) -> SearchAuthUserAccountFilter;
}

#[async_trait::async_trait]
pub trait SearchAuthUserAccountRepository {
    async fn search(
        &self,
        fields: SearchAuthUserAccountFilter,
    ) -> Result<AuthUserAccountSearch, RepositoryError>;
}
