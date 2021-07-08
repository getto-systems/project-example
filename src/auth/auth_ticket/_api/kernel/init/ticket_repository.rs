use chrono::{TimeZone, Utc};
use sqlx::{query, MySqlPool};

use crate::z_details::_api::mysql::helper::mysql_error;

use crate::auth::auth_ticket::_api::kernel::infra::AuthTicketRepository;

use crate::auth::auth_ticket::_api::kernel::data::{
    AuthDateTime, AuthTicket, ExpansionLimitDateTime,
};
use crate::z_details::_api::repository::data::RepositoryError;

pub struct MysqlAuthTicketRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> MysqlAuthTicketRepository<'a> {
    pub const fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl<'a> AuthTicketRepository for MysqlAuthTicketRepository<'a> {
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

    async fn expansion_limit(
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

#[cfg(test)]
pub mod test {
    use std::{collections::HashMap, sync::Mutex};

    use crate::z_details::_api::repository::helper::infra_error;

    use crate::auth::auth_ticket::_api::kernel::infra::AuthTicketRepository;

    use crate::auth::auth_ticket::_api::kernel::data::{
        AuthDateTime, AuthTicket, AuthTicketId, ExpansionLimitDateTime,
    };
    use crate::z_details::_api::repository::data::RepositoryError;

    pub struct MemoryAuthTicketRepository<'a> {
        store: &'a MemoryAuthTicketStore,
    }

    impl<'a> MemoryAuthTicketRepository<'a> {
        pub const fn new(store: &'a MemoryAuthTicketStore) -> Self {
            Self { store }
        }
    }

    pub type MemoryAuthTicketStore = Mutex<MemoryAuthTicketMap>;
    pub struct MemoryAuthTicketMap {
        ticket: HashMap<String, Entry>,
    }

    struct Entry {
        limit: ExpansionLimitDateTime,
    }

    impl MemoryAuthTicketMap {
        pub fn new() -> Self {
            Self {
                ticket: HashMap::new(),
            }
        }

        pub fn with_ticket(ticket_id: AuthTicketId, limit: ExpansionLimitDateTime) -> Self {
            let mut store = Self::new();
            store.ticket.insert(ticket_id.extract(), Entry { limit });
            store
        }

        pub fn to_store(self) -> MemoryAuthTicketStore {
            Mutex::new(self)
        }

        fn get(&self, ticket: &AuthTicket) -> Option<&Entry> {
            self.ticket.get(ticket.ticket_id_as_str())
        }
        fn insert(&mut self, ticket: AuthTicket, entry: Entry) {
            let ticket = ticket.extract();
            self.ticket.insert(ticket.ticket_id, entry);
        }
    }

    #[async_trait::async_trait]
    impl<'a> AuthTicketRepository for MemoryAuthTicketRepository<'a> {
        async fn issue(
            &self,
            ticket: AuthTicket,
            limit: ExpansionLimitDateTime,
            _issued_at: AuthDateTime,
        ) -> Result<(), RepositoryError> {
            let mut store = self.store.lock().unwrap();

            if store.get(&ticket).is_some() {
                return Err(infra_error("ticket id conflict"));
            }

            // 実際のデータベースには user_id と registered_at も保存する必要がある
            store.insert(ticket, Entry { limit });

            Ok(())
        }

        async fn discard(
            &self,
            auth_ticket: AuthTicket,
            _discard_at: AuthDateTime,
        ) -> Result<(), RepositoryError> {
            let mut store = self.store.lock().unwrap();

            store.ticket.remove(auth_ticket.ticket_id_as_str());

            // 実際のデータベースには discard_at も保存する必要がある

            Ok(())
        }

        async fn expansion_limit(
            &self,
            ticket: &AuthTicket,
        ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
            let store = self.store.lock().unwrap();

            Ok(store
                .ticket
                .get(ticket.ticket_id_as_str())
                .map(|entry| entry.limit.clone()))
        }
    }
}
