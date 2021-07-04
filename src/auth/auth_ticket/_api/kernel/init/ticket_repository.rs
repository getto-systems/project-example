use chrono::{NaiveDateTime, TimeZone, Utc};
use mysql::{params, prelude::Queryable, Pool};

use crate::z_details::_api::mysql::helper::infra_error;

use crate::auth::auth_ticket::_api::kernel::infra::AuthTicketRepository;

use crate::auth::auth_ticket::_api::kernel::data::{
    AuthDateTime, AuthTicket, ExpansionLimitDateTime,
};
use crate::z_details::_api::repository::data::RepositoryError;

pub struct MysqlAuthTicketRepository<'a> {
    pool: &'a Pool,
}

impl<'a> MysqlAuthTicketRepository<'a> {
    pub const fn new(pool: &'a Pool) -> Self {
        Self { pool }
    }
}

struct Ticket {
    user_id: String,
    ticket_id: String,
}

struct TicketExpansionLimit {
    expansion_limit: NaiveDateTime,
}

impl<'a> AuthTicketRepository for MysqlAuthTicketRepository<'a> {
    fn issue(
        &self,
        ticket: AuthTicket,
        expansion_limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        let mut conn = self.pool.get_conn().map_err(infra_error)?;

        let ticket = ticket.extract();

        conn.exec_drop(
            r"#####
            insert into ticket
                (user_id, ticket_id, expansion_limit, issued_at)
            values
                (:user_id, :ticket_id, :expansion_limit, :issued_at)
            #####",
            params! {
                "user_id" => ticket.user_id,
                "ticket_id" => ticket.ticket_id,
                "expansion_limit" => expansion_limit.extract().naive_utc(),
                "issued_at" => issued_at.extract().naive_utc(),
            },
        )
        .map_err(infra_error)?;

        Ok(())
    }

    fn discard(&self, ticket: AuthTicket, discard_at: AuthDateTime) -> Result<(), RepositoryError> {
        let mut conn = self.pool.get_conn().map_err(infra_error)?;
        let mut conn = conn
            .start_transaction(Default::default())
            .map_err(infra_error)?;

        let ticket = ticket.extract();
        let ticket_id = ticket.ticket_id;

        let found = conn
            .exec_map(
                r"#####
                select user_id, ticket_id from ticket
                where ticket_id = :ticket_id
                #####",
                params! {
                    "ticket_id" => &ticket_id,
                },
                |(user_id, ticket_id)| Ticket { user_id, ticket_id },
            )
            .map_err(infra_error)?;

        let found = found.get(0).ok_or_else(|| {
            RepositoryError::InfraError(format!("ticket not found; ticket-id: {}", &ticket_id))
        })?;

        conn.exec_drop(
            r"#####
            delete from ticket
            where ticket_id = :ticket_id
            #####",
            params! {
                "ticket_id" => &ticket_id,
            },
        )
        .map_err(infra_error)?;

        conn.exec_drop(
            r"#####
            insert into ticket_discarded
                (user_id, ticket_id, discard_at)
            values
                (:user_id, :ticket_id, :discard_at)
            #####",
            params! {
                "user_id" => &found.user_id,
                "ticket_id" => &found.ticket_id,
                "discard_at" => discard_at.extract().naive_utc(),
            },
        )
        .map_err(infra_error)?;

        conn.commit().map_err(infra_error)?;

        Ok(())
    }

    fn expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
        let mut conn = self.pool.get_conn().map_err(infra_error)?;

        let found = conn
            .exec_map(
                r"#####
                select expansion_limit from ticket
                where ticket_id = :ticket_id
                #####",
                params! {
                    "ticket_id" => ticket.ticket_id_as_str(),
                },
                |expansion_limit| TicketExpansionLimit { expansion_limit },
            )
            .map_err(infra_error)?;

        Ok(found.get(0).map(|found| {
            ExpansionLimitDateTime::restore(Utc.from_utc_datetime(&found.expansion_limit))
        }))
    }
}

#[cfg(test)]
pub mod test {
    use std::{collections::HashMap, sync::Mutex};

    use crate::z_details::_api::repository::helper::register_conflict_error;

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

    impl<'a> AuthTicketRepository for MemoryAuthTicketRepository<'a> {
        fn issue(
            &self,
            ticket: AuthTicket,
            limit: ExpansionLimitDateTime,
            _issued_at: AuthDateTime,
        ) -> Result<(), RepositoryError> {
            let mut store = self.store.lock().unwrap();

            if store.get(&ticket).is_some() {
                return Err(register_conflict_error(ticket.ticket_id_as_str()));
            }

            // 実際のデータベースには user_id と registered_at も保存する必要がある
            store.insert(ticket, Entry { limit });

            Ok(())
        }

        fn discard(
            &self,
            auth_ticket: AuthTicket,
            _discard_at: AuthDateTime,
        ) -> Result<(), RepositoryError> {
            let mut store = self.store.lock().unwrap();

            store.ticket.remove(auth_ticket.ticket_id_as_str());

            // 実際のデータベースには discard_at も保存する必要がある

            Ok(())
        }

        fn expansion_limit(
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
