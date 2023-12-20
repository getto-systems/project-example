use sqlx::{MySql, Pool, Transaction};

use crate::common::api::repository::{data::RepositoryError, infra::RepositoryTransaction};

pub struct MySqlRepositoryTransaction<'c> {
    conn: Transaction<'c, MySql>,
}

impl<'c> MySqlRepositoryTransaction<'c> {
    pub async fn begin(
        pool: &Pool<MySql>,
    ) -> Result<MySqlRepositoryTransaction<'c>, RepositoryError> {
        let conn = pool
            .begin()
            .await
            .map_err(|err| ("begin transaction", err))?;
        Ok(Self { conn })
    }
}

#[async_trait::async_trait]
impl<'c> RepositoryTransaction for MySqlRepositoryTransaction<'c> {
    type Connection = Transaction<'c, MySql>;

    fn conn(&mut self) -> &mut Self::Connection {
        &mut self.conn
    }
    async fn commit(self) -> Result<(), RepositoryError> {
        self.conn
            .commit()
            .await
            .map_err(|err| ("commit error", err))?;
        Ok(())
    }
}
