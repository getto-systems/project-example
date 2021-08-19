use crate::z_details::_common::repository::data::RepositoryError;

pub enum DiscardAuthTicketEvent {
    Success,
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "discard success";
const ERROR: &'static str = "discard error";

impl std::fmt::Display for DiscardAuthTicketEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "{}", SUCCESS),
            Self::RepositoryError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}
