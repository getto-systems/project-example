use sqlx::{query, MySqlPool};

use crate::z_lib::api::repository::mysql::helper::mysql_error;

use crate::auth::user::password::reset::request_token::api::infra::ResetTokenDestinationRepository;

use crate::{
    auth::user::{
        login_id::kernel::data::LoginId,
        password::reset::kernel::data::{ResetTokenDestination, ResetTokenDestinationExtract},
    },
    z_lib::api::repository::data::RepositoryError,
};

pub struct MysqlResetTokenDestinationRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> MysqlResetTokenDestinationRepository<'a> {
    pub const fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl<'a> ResetTokenDestinationRepository for MysqlResetTokenDestinationRepository<'a> {
    async fn get(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<ResetTokenDestination>, RepositoryError> {
        let conn = self.pool;

        let found = query!(
            r"#####
            select
                email
            from user_password_reset_token_destination
            inner join user on user.user_id = user_password_reset_token_destination.user_id
            where user.login_id = ?
            #####",
            login_id.as_str(),
        )
        .fetch_optional(conn)
        .await
        .map_err(mysql_error)?;

        Ok(found.map(|entry| ResetTokenDestinationExtract { email: entry.email }.restore()))
    }
}
