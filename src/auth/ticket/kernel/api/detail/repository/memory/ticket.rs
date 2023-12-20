use crate::auth::ticket::kernel::detail::repository::memory::StoreTicket;

use crate::auth::{
    kernel::data::{AuthDateTime, ExpansionLimitDateTime},
    ticket::kernel::data::{AuthTicket, AuthTicketId},
    user::kernel::data::AuthUserId,
};

pub struct MapTicket;

impl MapTicket {
    pub fn get_expansion_limit(
        store: &StoreTicket,
        ticket_id: &AuthTicketId,
    ) -> Option<ExpansionLimitDateTime> {
        let store = store.lock().unwrap();
        store
            .get(ticket_id)
            .map(|(_user_id, limit, _issued_at)| limit.clone())
    }

    pub fn insert_entry(
        store: &StoreTicket,
        ticket_id: AuthTicketId,
        entry: (AuthUserId, ExpansionLimitDateTime, AuthDateTime),
    ) {
        let mut store = store.lock().unwrap();
        store.insert(ticket_id, entry);
    }
    pub fn insert_ticket(
        store: &StoreTicket,
        ticket: AuthTicket,
        limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) {
        // 本当のデータベースでは ticket_id がすでに存在したらエラーにする
        let mut store = store.lock().unwrap();
        store.insert(ticket.ticket_id, (ticket.attrs.user_id, limit, issued_at));
    }

    pub fn remove_ticket(store: &StoreTicket, ticket_id: &AuthTicketId) {
        let mut store = store.lock().unwrap();
        store.remove(ticket_id);
    }

    pub fn get_all_ticket_id(store: &StoreTicket, user_id: &AuthUserId) -> Vec<AuthTicketId> {
        let user_id = user_id.clone();
        let store = store.lock().unwrap();
        store
            .iter()
            .filter_map(|(ticket_id, (entry_user_id, _limit, _issued_at))| {
                if *entry_user_id == user_id {
                    Some(ticket_id.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
