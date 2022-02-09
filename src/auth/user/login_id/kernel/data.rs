#[derive(Clone)]
pub struct LoginId(String);

impl LoginId {
    pub fn validate(login_id: impl LoginIdExtract) -> Result<Self, ValidateLoginIdError> {
        Ok(Self(login_id.validate()?))
    }

    pub(in crate::auth) const fn restore(login_id: String) -> Self {
        Self(login_id)
    }

    pub fn extract(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub trait LoginIdExtract {
    fn validate(self) -> Result<String, ValidateLoginIdError>;
}

pub struct LoginIdBasket(String);

impl LoginIdBasket {
    pub fn new(login_id: String) -> Self {
        Self(login_id)
    }

    pub fn extract(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub enum ValidateLoginIdError {
    Empty,
    TooLong,
}

impl std::fmt::Display for ValidateLoginIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Empty => write!(f, "empty login id"),
            Self::TooLong => write!(f, "too long login id"),
        }
    }
}
