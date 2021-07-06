use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use super::convert::validate_login_id;

#[derive(Clone)]
pub struct LoginId(String);

impl LoginId {
    pub fn validate(login_id: String) -> Result<Self, ValidateLoginIdError> {
        validate_login_id(&login_id)?;
        Ok(Self(login_id))
    }

    pub const fn restore(login_id: String) -> Self {
        Self(login_id)
    }

    pub fn extract(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug)]
pub enum ValidateLoginIdError {
    Empty,
    TooLong,
}

impl Display for ValidateLoginIdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Empty => write!(f, "empty login id"),
            Self::TooLong => write!(f, "too long login id"),
        }
    }
}
impl Error for ValidateLoginIdError {}
