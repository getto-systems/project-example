use crate::{
    auth::user::{kernel::data::GrantedAuthRolesBasket, login_id::kernel::data::LoginIdBasket},
    z_lib::search::data::{SearchPage, SearchSort, SearchSortExtract},
};

pub struct SearchAuthUserAccountFilter {
    offset: i32,
    sort: SearchSort<SearchAuthUserAccountSortKey>,
    login_id: Option<String>,
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
    pub fn login_id(&self) -> &Option<String> {
        &self.login_id
    }
}

pub struct SearchAuthUserAccountFilterExtract {
    pub offset: i32,
    pub sort: SearchSortExtract,
    pub login_id: Option<String>,
}
impl Into<SearchAuthUserAccountFilter> for SearchAuthUserAccountFilterExtract {
    fn into(self) -> SearchAuthUserAccountFilter {
        SearchAuthUserAccountFilter {
            offset: self.offset,
            sort: self.sort.into(),
            login_id: self.login_id,
        }
    }
}

pub struct SearchAuthUserAccountBasket {
    pub page: SearchPage,
    pub sort: SearchSort<SearchAuthUserAccountSortKey>,
    pub users: Vec<AuthUserAccountBasket>,
}
pub struct AuthUserAccountBasket {
    pub login_id: LoginIdBasket,
    pub granted_roles: GrantedAuthRolesBasket,
}
