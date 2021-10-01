use crate::{
    auth::remote::data::ValidateApiTokenError,
    example::outline::remote::get_menu_badge::data::OutlineMenuBadge,
    z_lib::remote::repository::data::RepositoryError,
};

pub enum GetOutlineMenuBadgeEvent {
    Success(OutlineMenuBadge),
    ValidateError(ValidateApiTokenError),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "get menu badge success";
const ERROR: &'static str = "get menu badge error";

impl std::fmt::Display for GetOutlineMenuBadgeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::ValidateError(err) => write!(f, "{}: {}", ERROR, err),
            Self::RepositoryError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}