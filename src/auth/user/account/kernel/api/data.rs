use crate::auth::user::{kernel::data::GrantedAuthRoles, login_id::kernel::data::LoginId};

pub struct AuthUserAccount {
    pub login_id: LoginId,
    pub granted_roles: GrantedAuthRoles,
}
