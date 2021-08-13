use crate::auth::{
    auth_ticket::_common::kernel::data::AuthTokenEncoded,
    auth_user::_common::kernel::data::AuthUserExtract,
};

pub struct AuthTicketEncoded {
    pub user: AuthUserExtract,
    pub token: AuthTokenEncoded,
}
