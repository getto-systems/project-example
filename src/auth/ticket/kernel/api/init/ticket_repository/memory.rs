
    use std::{collections::HashMap, sync::Mutex};

    use crate::z_lib::api::repository::helper::infra_error;

    use crate::auth::ticket::{
        encode::infra::EncodeAuthTicketRepository, issue::infra::IssueAuthTicketRepository,
        logout::api::infra::LogoutAuthTicketRepository,
    };

    use crate::{
        auth::ticket::kernel::api::data::{
            AuthDateTime, AuthTicket, AuthTicketId, ExpansionLimitDateTime,
        },
        z_lib::api::repository::data::RepositoryError,
    };

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
    impl<'a> IssueAuthTicketRepository for MemoryAuthTicketRepository<'a> {
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
    }

    #[async_trait::async_trait]
    impl<'a> LogoutAuthTicketRepository for MemoryAuthTicketRepository<'a> {
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
    }

    #[async_trait::async_trait]
    impl<'a> EncodeAuthTicketRepository for MemoryAuthTicketRepository<'a> {
        async fn find_expansion_limit(
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