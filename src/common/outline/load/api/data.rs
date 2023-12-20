use crate::x_content::menu::badge::OutlineMenuBadgePath;

use crate::{auth::data::AuthorizeProxyError, common::api::repository::data::RepositoryError};

pub struct OutlineMenuBadge(Vec<(OutlineMenuBadgePath, OutlineMenuBadgeCount)>);

impl OutlineMenuBadge {
    pub fn new(counts: Vec<(OutlineMenuBadgePath, OutlineMenuBadgeCount)>) -> Self {
        Self(counts)
    }

    pub fn extract(self) -> Vec<(OutlineMenuBadgePath, OutlineMenuBadgeCount)> {
        self.0
    }
}

impl std::fmt::Display for OutlineMenuBadge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OutlineMenuBadgeCount(i32);

impl OutlineMenuBadgeCount {
    pub const fn restore(count: i32) -> Self {
        Self(count)
    }

    pub const fn extract(self) -> i32 {
        self.0
    }
}

#[derive(Debug)]
pub enum LoadOutlineMenuBadgeError {
    RepositoryError(RepositoryError),
    AuthorizeProxyError(AuthorizeProxyError),
}

const ERROR: &'static str = "load outline-menu-badge error";

impl std::fmt::Display for LoadOutlineMenuBadgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::AuthorizeProxyError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<RepositoryError> for LoadOutlineMenuBadgeError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

impl From<AuthorizeProxyError> for LoadOutlineMenuBadgeError {
    fn from(value: AuthorizeProxyError) -> Self {
        Self::AuthorizeProxyError(value)
    }
}
