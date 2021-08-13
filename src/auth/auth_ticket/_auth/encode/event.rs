use std::fmt::Display;

use crate::{
    auth::auth_ticket::{
        _auth::encode::data::{AuthTokenExpires, EncodeAuthTokenError},
        _common::encode::data::AuthTicketEncoded,
    },
    z_details::_common::repository::data::RepositoryError,
};

pub enum EncodeAuthTicketEvent {
    TokenExpiresCalculated(AuthTokenExpires),
    Success(AuthTicketEncoded),
    TicketNotFound,
    RepositoryError(RepositoryError),
    EncodeError(EncodeAuthTokenError),
}

const SUCCESS: &'static str = "encode success";
const ERROR: &'static str = "encode error";

impl Display for EncodeAuthTicketEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TokenExpiresCalculated(expires) => {
                write!(f, "token expires calculated; {}", expires)
            }
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::TicketNotFound => write!(f, "{}; ticket data not found", ERROR),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::EncodeError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}
