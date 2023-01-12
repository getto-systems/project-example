use std::{collections::HashMap, sync::Mutex};

use crate::auth::{
    kernel::data::{AuthDateTime, ExpansionLimitDateTime},
    ticket::kernel::data::{AuthTicket, AuthTicketId},
    user::kernel::data::AuthUserId,
};

pub struct MapTicket<'a> {
    store: &'a StoreTicket,
}
pub type StoreTicket = Mutex<HashMap<AuthTicketId, EntryTicket>>;

pub struct EntryTicket {
    pub user_id: AuthUserId,
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
        // 本当のデータベースでは ticket_id がすでに存在したらエラーにする
        let mut store = self.store.lock().unwrap();
        store.insert(
            ticket.ticket_id,
            EntryTicket {
                user_id: ticket.attrs.user_id,
                limit,
                issued_at,
            },
        );
    }

    pub fn remove_ticket(&self, ticket_id: &AuthTicketId) {
        let mut store = self.store.lock().unwrap();
        store.remove(ticket_id);
    }

    pub fn get_all_ticket_id(&self, user_id: &AuthUserId) -> Vec<AuthTicketId> {
        let user_id = user_id.clone();
        let store = self.store.lock().unwrap();
        store
            .iter()
            .filter_map(|(ticket_id, entry)| {
                if entry.user_id == user_id {
                    Some(ticket_id.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
