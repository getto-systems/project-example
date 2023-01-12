use jsonwebtoken::{Algorithm, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::auth::{
    kernel::data::ExpireDateTime, user::password::reset::kernel::data::ResetPasswordId,
};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: i64,
}

impl Claims {
    const fn algorithm() -> Algorithm {
        Algorithm::ES384
    }

    pub fn new(id: ResetPasswordId, expires: ExpireDateTime) -> Self {
        Self {
            sub: id.extract(),
            exp: expires.clone().extract_to_timestamp(),
        }
    }

    pub fn validation() -> Validation {
        Validation::new(Self::algorithm())
    }

    pub fn header(&self) -> Header {
        Header::new(Self::algorithm())
    }
}

impl Into<ResetPasswordId> for Claims {
    fn into(self) -> ResetPasswordId {
        ResetPasswordId::restore(self.sub)
    }
}
