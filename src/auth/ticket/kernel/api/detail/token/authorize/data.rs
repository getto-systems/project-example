use std::collections::HashSet;

use jsonwebtoken::{Algorithm, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::auth::{kernel::data::ExpireDateTime, ticket::kernel::data::AuthTicket};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    exp: i64,
    pub ticket_id: String,
    pub user_id: String,
    pub granted: HashSet<String>,
}

impl Claims {
    const fn algorithm() -> Algorithm {
        Algorithm::ES384
    }
    const fn audience() -> &'static str {
        "authorize"
    }

    pub fn new(ticket: AuthTicket, expires: ExpireDateTime) -> Self {
        Self {
            aud: Self::audience().to_owned(),
            exp: expires.extract_to_timestamp(),
            ticket_id: ticket.ticket_id.extract(),
            user_id: ticket.attrs.user_id.extract(),
            granted: ticket.attrs.granted.extract(),
        }
    }

    pub fn validation() -> Validation {
        let mut validation = Validation::new(Self::algorithm());
        validation.set_audience(&[Self::audience()]);
        validation
    }

    pub fn header(&self) -> Header {
        Header::new(Self::algorithm())
    }
}
