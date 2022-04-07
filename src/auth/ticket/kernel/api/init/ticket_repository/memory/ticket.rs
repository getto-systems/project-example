use std::{collections::HashMap, sync::Mutex};

use crate::auth::{
    ticket::kernel::data::{AuthDateTime, AuthTicket, AuthTicketId, ExpansionLimitDateTime},
    user::kernel::data::AuthUser,
};

pub struct MapTicket<'a> {
    store: &'a StoreTicket,
}
pub type StoreTicket = Mutex<HashMap<AuthTicketId, EntryTicket>>;

pub struct EntryTicket {
    pub user: AuthUser,
    pub limit: ExpansionLimitDateTime,
    pub issued_at: AuthDateTime,
}

impl<'a> MapTicket<'a> {
    pub fn new_store() -> StoreTicket {
        Mutex::new(HashMap::new())
    }
    pub const fn new(store: &'a StoreTicket) -> Self {
        Self { store }
    }

    pub fn get_expansion_limit(&self, ticket_id: &AuthTicketId) -> Option<ExpansionLimitDateTime> {
        let store = self.store.lock().unwrap();
        store.get(ticket_id).map(|entry| entry.limit.clone())
    }

    pub fn insert_entry(&self, ticket_id: AuthTicketId, entry: EntryTicket) {
        let mut store = self.store.lock().unwrap();
        store.insert(ticket_id, entry);
    }
    pub fn insert_ticket(
        &self,
        ticket: AuthTicket,
        limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) {
        let (ticket_id, user) = ticket.extract();

        // 本当のデータベースでは ticket_id がすでに存在したらエラーにする
        let mut store = self.store.lock().unwrap();
        store.insert(
            ticket_id,
            EntryTicket {
                user,
                limit,
                issued_at,
            },
        );
    }

    pub fn remove_ticket(&self, ticket_id: &AuthTicketId) {
        let mut store = self.store.lock().unwrap();
        store.remove(ticket_id);
    }
}
