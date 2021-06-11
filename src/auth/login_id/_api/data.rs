use std::{
    error::Error,
    fmt::{Display, Formatter},
};

pub struct LoginId(String);

// login id には技術的な制限はないが、使用可能な最大文字数（バイト数ではない）は定義しておく
// ui の設定と同期させること
const LOGIN_ID_MAX_LENGTH: usize = 100;

impl LoginId {
    pub fn validate(login_id: String) -> Result<Self, ConvertLoginIdError> {
        match login_id.chars().count() {
            n if n == 0 => Err(ConvertLoginIdError::Empty),
            n if n > LOGIN_ID_MAX_LENGTH => Err(ConvertLoginIdError::TooLong),
            _ => Ok(Self(login_id)),
        }
    }

    pub fn extract(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug)]
pub enum ConvertLoginIdError {
    Empty,
    TooLong,
}

impl Display for ConvertLoginIdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Empty => write!(f, "empty login id"),
            Self::TooLong => write!(f, "too long login id"),
        }
    }
}
impl Error for ConvertLoginIdError {}
