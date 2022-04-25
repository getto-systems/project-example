use crate::{
    auth::user::{
        kernel::data::GrantedAuthRoles, login_id::kernel::data::LoginId,
        password::reset::kernel::data::ResetTokenDestination,
    },
    z_lib::validate::data::ValidateTextError,
};

pub struct AuthUserAccount {
    pub login_id: LoginId,
    pub granted_roles: GrantedAuthRoles,
    pub reset_token_destination: ResetTokenDestination,
    pub attrs: AuthUserAttributes,
}

impl std::fmt::Display for AuthUserAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}; [{}]; {}; {}",
            self.login_id, self.granted_roles, self.reset_token_destination, self.attrs,
        )
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct AuthUserAttributes(AuthUserAttributesExtract);

impl AuthUserAttributes {
    pub fn convert(
        attrs: AuthUserAttributesExtract,
    ) -> Result<Self, ValidateAuthUserAttributesError> {
        Ok(Self(attrs.convert()?))
    }

    pub(in crate::auth) const fn restore(attrs: AuthUserAttributesExtract) -> Self {
        Self(attrs)
    }

    pub fn extract(self) -> AuthUserAttributesExtract {
        self.0
    }
}

impl std::fmt::Display for AuthUserAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct AuthUserAttributesExtract {
    pub memo: String,
}

impl Default for AuthUserAttributesExtract {
    fn default() -> Self {
        Self { memo: "".into() }
    }
}

impl std::fmt::Display for AuthUserAttributesExtract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "memo: {}", self.memo)
    }
}

pub enum ValidateAuthUserAttributesError {
    Memo(ValidateTextError),
}

impl std::fmt::Display for ValidateAuthUserAttributesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Memo(err) => write!(f, "memo: {}", err),
        }
    }
}
