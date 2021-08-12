use std::fmt::Display;

use crate::{
    auth::auth_ticket::{
        _auth::kernel::data::AuthTicket, _common::kernel::data::ExpansionLimitDateTime,
    },
    z_details::_common::repository::data::RepositoryError,
};

pub enum IssueAuthTicketEvent {
    ExpansionLimitCalculated(ExpansionLimitDateTime),
    Success(AuthTicket),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "issue success";
const ERROR: &'static str = "issue error";

impl Display for IssueAuthTicketEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpansionLimitCalculated(limit) => {
                write!(f, "expansion limit calculated; {}", limit)
            }
            Self::Success(ticket) => write!(f, "{}; {}", SUCCESS, ticket),
            Self::RepositoryError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}
