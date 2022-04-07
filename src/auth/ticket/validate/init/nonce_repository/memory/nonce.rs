use std::{collections::HashMap, sync::Mutex};

use crate::{
    auth::ticket::kernel::data::{AuthDateTime, AuthNonce, ExpireDateTime},
    z_lib::repository::data::RegisterResult,
};

pub struct MapNonce {
    store: Mutex<HashMap<AuthNonce, EntryNonce>>,
}

pub struct EntryNonce {
    pub expires: ExpireDateTime,
    pub registered_at: AuthDateTime,
}

impl MapNonce {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }

    pub fn insert_entry(&self, nonce: AuthNonce, entry: EntryNonce) {
        let mut store = self.store.lock().unwrap();
        store.insert(nonce, entry);
    }
    pub fn insert_nonce(
        &self,
        nonce: AuthNonce,
        expires: ExpireDateTime,
        registered_at: AuthDateTime,
    ) -> RegisterResult<()> {
        let mut store = self.store.lock().unwrap();
        if store.get(&nonce).is_some() {
            return RegisterResult::Conflict;
        }

        store.insert(
            nonce,
            EntryNonce {
                expires,
                registered_at,
            },
        );
        RegisterResult::Success(())
    }
}
