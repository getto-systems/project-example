use crate::{outline::_common::data::OutlineMenuBadge, z_details::_common::repository::data::RepositoryError};

pub enum GetOutlineMenuBadgeEvent {
    Success(OutlineMenuBadge),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "get menu badge success";
const ERROR: &'static str = "get menu badge error";

impl std::fmt::Display for GetOutlineMenuBadgeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::RepositoryError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}
