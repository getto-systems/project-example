use crate::auth::data::AuthPermissionRequired;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthPermission {
    AuthUser,
}

impl AuthPermission {
    pub const fn variants() -> [Self; 1] {
        [
            // variants が増えたらここにも追加する
            Self::AuthUser,
        ]
    }

    pub fn convert(value: impl AuthPermissionExtract) -> Option<Self> {
        value.convert()
    }

    pub fn extract(self) -> String {
        self.as_str().to_owned()
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::AuthUser => "auth-user",
        }
    }
}

pub trait AuthPermissionExtract {
    fn convert(self) -> Option<AuthPermission>;
}

impl AuthPermissionExtract for String {
    fn convert(self) -> Option<AuthPermission> {
        convert_permission(self.as_str())
    }
}

impl AuthPermissionExtract for &str {
    fn convert(self) -> Option<AuthPermission> {
        convert_permission(self)
    }
}

fn convert_permission(value: &str) -> Option<AuthPermission> {
    for key in AuthPermission::variants() {
        if value == key.as_str() {
            return Some(key);
        }
    }

    None
}

impl AuthPermissionRequired {
    pub fn user() -> Self {
        Self::HasSome(vec![AuthPermission::AuthUser].into_iter().collect())
    }
}
