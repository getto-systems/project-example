use crate::auth::{
    auth_ticket::_common::kernel::data::AuthTokenEncoded,
    auth_user::_common::kernel::data::{AuthUser, AuthUserExtract},
};

pub struct EncodeAuthTicketResponse {
    user: AuthUser,
    token: AuthTokenEncoded,
}

impl EncodeAuthTicketResponse {
    pub const fn new(user: AuthUser, token: AuthTokenEncoded) -> Self {
        Self { user, token }
    }

    pub fn extract(self) -> (AuthUserExtract, AuthTokenEncoded) {
        (self.user.extract(), self.token)
    }
}
