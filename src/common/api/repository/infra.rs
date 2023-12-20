use crate::common::api::repository::data::RepositoryError;

#[async_trait::async_trait]
pub trait CurrentlyInUseChecker<T: RepositoryTransaction, M>: Send + Sync {
    async fn check_currently_in_use(&self, conn: &mut T, id: &M) -> Result<bool, RepositoryError>;
    fn used_by(&self) -> String;
}

#[async_trait::async_trait]
pub trait RepositoryTransaction {
    type Connection;

    fn conn(&mut self) -> &mut Self::Connection;
    async fn commit(self) -> Result<(), RepositoryError>;
}
