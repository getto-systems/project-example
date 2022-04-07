use crate::auth::user::{
    kernel::data::GrantedAuthRoles, login_id::kernel::data::LoginId,
    password::reset::kernel::data::ResetTokenDestination,
};

pub struct AuthUserAccount {
    pub login_id: LoginId,
    pub granted_roles: GrantedAuthRoles,
    pub reset_token_destination: ResetTokenDestination,
}

impl std::fmt::Display for AuthUserAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}; [{}]; {}",
            self.login_id, self.granted_roles, self.reset_token_destination
        )
    }
}
