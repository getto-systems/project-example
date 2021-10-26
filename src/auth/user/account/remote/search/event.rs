use crate::auth::ticket::remote::validate::event::ValidateAuthTokenEvent;

use crate::{
    auth::user::account::remote::search::data::SearchAuthUserAccountBasket,
    z_lib::remote::repository::data::RepositoryError,
};

pub enum SearchAuthUserAccountEvent {
    Success(SearchAuthUserAccountBasket),
    Validate(ValidateAuthTokenEvent),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "search user account success";
const ERROR: &'static str = "search user account error";

impl std::fmt::Display for SearchAuthUserAccountEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::Validate(event) => event.fmt(f),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}
