use crate::x_content::permission::AuthPermission;

use crate::{
    auth::{
        ticket::kernel::data::AuthPermissionGranted,
        user::{
            account::kernel::data::AuthUserAccount,
            login_id::kernel::data::{LoginId, SearchLoginId},
        },
    },
    common::api::search::data::{SearchPage, SearchProps, SearchSort},
};

pub struct SearchAuthUserAccountFilter {
    pub search: SearchProps<SearchAuthUserAccountSortKey>,
    pub props: SearchAuthUserAccountFilterProps,
}

pub struct SearchAuthUserAccountFilterProps {
    pub login_id: SearchLoginId,
    pub granted: Vec<AuthPermission>,
}

impl SearchAuthUserAccountFilterProps {
    pub fn is_match(&self, model: &AuthUserAccount) -> bool {
        self.match_login_id(&model.login_id) && self.match_granted(&model.attrs.granted)
    }
    fn match_login_id(&self, login_id: &LoginId) -> bool {
        login_id == &self.login_id
    }
    fn match_granted(&self, granted: &AuthPermissionGranted) -> bool {
        if self.granted.is_empty() {
            return true;
        }
        granted.all_contains(&self.granted)
    }
}

#[derive(Clone, Copy)]
pub enum SearchAuthUserAccountSortKey {
    // 変更したら variants() も更新する
    LoginId,
}

impl SearchAuthUserAccountSortKey {
    pub const fn variants() -> [Self; 1] {
        [
            // variants が増えたらここにも追加する
            Self::LoginId,
        ]
    }

    pub fn convert(value: String) -> Option<Self> {
        for key in Self::variants() {
            if key.extract() == value {
                return Some(key);
            }
        }

        None
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::LoginId => "login-id",
        }
    }
    pub fn extract(self) -> String {
        self.as_str().to_owned()
    }
}

impl Default for SearchAuthUserAccountSortKey {
    fn default() -> Self {
        Self::LoginId
    }
}

impl From<String> for SearchAuthUserAccountSortKey {
    fn from(value: String) -> Self {
        Self::convert(value).unwrap_or(Default::default())
    }
}

impl Into<String> for SearchAuthUserAccountSortKey {
    fn into(self) -> String {
        self.extract()
    }
}

pub struct AuthUserAccountSearch {
    pub page: SearchPage,
    pub sort: SearchSort<SearchAuthUserAccountSortKey>,
    pub users: Vec<AuthUserAccount>,
}
