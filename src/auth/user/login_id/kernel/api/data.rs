use crate::z_lib::validate::data::ValidateTextError;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LoginId(String);

impl LoginId {
    pub fn convert(login_id: impl LoginIdExtract) -> Result<Self, ValidateLoginIdError> {
        Ok(Self(login_id.convert()?))
    }

    pub(in crate::auth) const fn restore(login_id: String) -> Self {
        Self(login_id)
    }

    pub fn extract(self) -> String {
        self.0
    }

    pub fn inner(&self) -> &String {
        &self.0
    }
}

impl std::fmt::Display for LoginId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "login id: {}", self.0)
    }
}

pub trait LoginIdExtract {
    fn convert(self) -> Result<String, ValidateLoginIdError>;
}

pub enum ValidateLoginIdError {
    Text(ValidateTextError),
}

impl std::fmt::Display for ValidateLoginIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Text(err) => err.fmt(f),
        }
    }
}
