use crate::common::api::validate::data::ValidateTextError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoginId(String);

impl LoginId {
    pub fn convert(value: impl LoginIdExtract) -> Result<Self, ValidateLoginIdError> {
        Ok(Self(
            value.convert().map_err(ValidateLoginIdError::LoginId)?,
        ))
    }

    pub(in crate::auth) const fn restore(value: String) -> Self {
        Self(value)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

impl std::fmt::Display for LoginId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "login-id: {}", self.0)
    }
}

pub trait LoginIdExtract {
    fn convert(self) -> Result<String, ValidateTextError>;
}

#[derive(Debug)]
pub enum ValidateLoginIdError {
    LoginId(ValidateTextError),
}

impl std::fmt::Display for ValidateLoginIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::LoginId(err) => write!(f, "login-id: {}", err),
        }
    }
}

pub struct SearchLoginId(Option<String>);

impl SearchLoginId {
    pub(in crate::auth) const fn restore(value: Option<String>) -> Self {
        Self(value)
    }
}

impl PartialEq<SearchLoginId> for LoginId {
    fn eq(&self, other: &SearchLoginId) -> bool {
        match other.0 {
            None => true,
            Some(ref value) => &self.0 == value,
        }
    }

    fn ne(&self, other: &SearchLoginId) -> bool {
        match other.0 {
            None => false,
            Some(ref value) => &self.0 != value,
        }
    }
}
