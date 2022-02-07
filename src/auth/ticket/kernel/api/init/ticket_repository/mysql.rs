use chrono::{TimeZone, Utc};
use sqlx::{query, MySqlPool};

use crate::z_lib::api::repository::mysql::helper::mysql_error;

use crate::auth::ticket::{
    encode::infra::EncodeAuthTicketRepository, issue::infra::IssueAuthTicketRepository,
    logout::api::infra::LogoutAuthTicketRepository,
};

use crate::{
    auth::ticket::kernel::api::data::{AuthDateTime, AuthTicket, ExpansionLimitDateTime},
    z_lib::api::repository::data::RepositoryError,
};

pub struct MysqlAuthTicketRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> MysqlAuthTicketRepository<'a> {
    pub const fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl<'a> IssueAuthTicketRepository for MysqlAuthTicketRepository<'a> {
    async fn issue(
        &self,
        ticket: AuthTicket,
        expansion_limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        let conn = self.pool;

        let ticket = ticket.extract();

        query!(
            r"#####
            insert into ticket
                (user_id, ticket_id, expansion_limit, issued_at)
            values
                (?, ?, ?, ?)
            #####",
            ticket.user_id,
            ticket.ticket_id,
            expansion_limit.extract(),
            issued_at.extract(),
        )
        .execute(conn)
        .await
        .map_err(mysql_error)?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> LogoutAuthTicketRepository for MysqlAuthTicketRepository<'a> {
    async fn discard(
        &self,
        ticket: AuthTicket,
        discard_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        let mut conn = self.pool.begin().await.map_err(mysql_error)?;

        let ticket = ticket.extract();
        let ticket_id = ticket.ticket_id;
        let user_id = ticket.user_id;

        let found = query!(
            r"#####
            select
                count(*) as count
            from ticket
            where ticket_id = ?
            and user_id = ?
            #####",
            &ticket_id,
            &user_id,
        )
        .fetch_one(&mut conn)
        .await
        .map_err(mysql_error)?;

        if found.count > 0 {
            query!(
                r"#####
                delete from ticket
                where ticket_id = ?
                #####",
                &ticket_id,
            )
            .execute(&mut conn)
            .await
            .map_err(mysql_error)?;

            query!(
                r"#####
                insert into ticket_discarded
                    (user_id, ticket_id, discard_at)
                values
                    (?, ?, ?)
                #####",
                &user_id,
                &ticket_id,
                discard_at.extract(),
            )
            .execute(&mut conn)
            .await
            .map_err(mysql_error)?;

            conn.commit().await.map_err(mysql_error)?;
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> EncodeAuthTicketRepository for MysqlAuthTicketRepository<'a> {
    async fn find_expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
        let conn = self.pool;

        let found = query!(
            r"#####
            select expansion_limit from ticket
            where ticket_id = ?
            #####",
            ticket.ticket_id_as_str(),
        )
        .fetch_optional(conn)
        .await
        .map_err(mysql_error)?;

        Ok(found.map(|found| {
            ExpansionLimitDateTime::restore(Utc.from_utc_datetime(&found.expansion_limit))
        }))
    }
}
