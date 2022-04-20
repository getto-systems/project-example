use crate::auth::user::kernel::data::GrantedAuthRoles;
use crate::auth::user::login_id::kernel::data::LoginId;
use crate::x_content::role::AuthRole;

use crate::{
    auth::user::account::kernel::data::AuthUserAccount,
    z_lib::search::data::{SearchPage, SearchSort, SearchSortExtract},
};

pub struct SearchAuthUserAccountFilter {
    offset: i32,
    sort: SearchSort<SearchAuthUserAccountSortKey>,
    login_id: Option<String>,
    granted_roles: Vec<AuthRole>,
}

pub enum SearchAuthUserAccountSortKey {
    LoginId,
}

impl From<String> for SearchAuthUserAccountSortKey {
    fn from(key: String) -> Self {
        match key.as_str() {
            "login-id" => Self::LoginId,
            _ => Self::LoginId,
        }
    }
}

impl Into<String> for SearchAuthUserAccountSortKey {
    fn into(self) -> String {
        match self {
            Self::LoginId => "login-id".to_owned(),
        }
    }
}

impl SearchAuthUserAccountFilter {
    pub fn offset(&self) -> i32 {
        self.offset
    }
    pub fn sort(&self) -> &SearchSort<SearchAuthUserAccountSortKey> {
        &self.sort
    }
    pub fn into_sort(self) -> SearchSort<SearchAuthUserAccountSortKey> {
        self.sort
    }

    pub fn match_login_id(&self, login_id: &LoginId) -> bool {
        match self.login_id {
            None => true,
            Some(ref filter_login_id) => login_id.as_str() == filter_login_id,
        }
    }
    pub fn match_granted_roles(&self, granted_roles: &Option<GrantedAuthRoles>) -> bool {
        if self.granted_roles.is_empty() {
            return true
        }
        match granted_roles {
            None => false,
            Some(granted_roles) => {
                let granted_roles = granted_roles.inner();
                self.granted_roles.iter().any(|role| granted_roles.contains(role))
            }
        }
    }
}

pub struct SearchAuthUserAccountFilterExtract {
    pub offset: i32,
    pub sort: SearchSortExtract,
    pub login_id: Option<String>,
    pub granted_roles: Vec<String>,
}
impl Into<SearchAuthUserAccountFilter> for SearchAuthUserAccountFilterExtract {
    fn into(self) -> SearchAuthUserAccountFilter {
        SearchAuthUserAccountFilter {
            offset: self.offset,
            sort: self.sort.into(),
            login_id: self.login_id,
            granted_roles: self
                .granted_roles
                .into_iter()
                .filter_map(|role| AuthRole::member(&role))
                .collect(),
        }
    }
}

pub struct AuthUserAccountSearch {
    pub page: SearchPage,
    pub sort: SearchSort<SearchAuthUserAccountSortKey>,
    pub users: Vec<AuthUserAccount>,
}
