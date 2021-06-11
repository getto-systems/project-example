use std::fmt::Display;

use super::super::kernel::data::AuthTicket;
use crate::z_details::_api::repository::data::RepositoryError;

pub enum IssueAuthTicketEvent {
    Success(AuthTicket),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "issue success";
const ERROR: &'static str = "issue error";

impl Display for IssueAuthTicketEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(ticket) => write!(f, "{}; {}", SUCCESS, ticket),
            Self::RepositoryError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}
