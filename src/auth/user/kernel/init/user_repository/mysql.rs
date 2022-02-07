use std::{collections::HashSet, iter::FromIterator};

use sqlx::{query, MySqlPool};

use crate::z_lib::api::repository::mysql::helper::mysql_error;

use crate::auth::user::kernel::infra::AuthUserRepository;

use crate::{
    auth::user::kernel::data::{AuthUser, AuthUserExtract, AuthUserId},
    z_lib::api::repository::data::RepositoryError,
};

pub struct MysqlAuthUserRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> MysqlAuthUserRepository<'a> {
    pub const fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl<'a> AuthUserRepository for MysqlAuthUserRepository<'a> {
    async fn get(&self, user_id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError> {
        // granted roles だけの検索だと、未登録だった場合に不足
        // user の存在を確認して、問題なければ granted roles を合わせて返す
        // group concat を使えば一度に取れるが、データの構築をしないといけない
        // ここでは効率を重視せずに、クエリを 2回投げることにする

        let mut conn = self.pool.begin().await.map_err(mysql_error)?;

        let found = query!(
            r"#####
            select
                count(*) as count
            from user
            where user_id = ?
            #####",
            user_id.as_str(),
        )
        .fetch_one(&mut conn)
        .await
        .map_err(mysql_error)?;

        if found.count == 0 {
            return Ok(None);
        }

        let roles = query!(
            r"#####
            select
                role
            from user_granted_role
            where user_id = ?
            #####",
            user_id.as_str(),
        )
        .fetch_all(&mut conn)
        .await
        .map_err(mysql_error)?;

        Ok(Some(
            AuthUserExtract {
                user_id: user_id.as_str().into(),
                granted_roles: HashSet::from_iter(roles.into_iter().map(|entry| entry.role)),
            }
            .restore(),
        ))
    }
}
