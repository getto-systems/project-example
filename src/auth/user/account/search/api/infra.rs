use crate::{
    auth::user::account::search::data::{
        SearchAuthUserAccountBasket, SearchAuthUserAccountFilter,
        SearchAuthUserAccountFilterExtract,
    },
    z_lib::repository::data::RepositoryError,
};

pub trait SearchAuthUserAccountRequestDecoder {
    fn decode(self) -> SearchAuthUserAccountFilterExtract;
}

#[async_trait::async_trait]
pub trait SearchAuthUserAccountRepository {
    async fn search(
        &self,
        fields: SearchAuthUserAccountFilter,
    ) -> Result<SearchAuthUserAccountBasket, RepositoryError>;
}
