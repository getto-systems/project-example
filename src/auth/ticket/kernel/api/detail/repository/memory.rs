pub mod ticket;

use std::{collections::HashMap, sync::Mutex};

use crate::auth::{
    kernel::data::{AuthDateTime, ExpansionLimitDateTime},
    ticket::kernel::data::AuthTicketId,
    user::kernel::data::AuthUserId,
};

pub type StoreTicket =
    Mutex<HashMap<AuthTicketId, (AuthUserId, ExpansionLimitDateTime, AuthDateTime)>>;
