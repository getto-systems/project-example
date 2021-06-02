use std::{
    convert::TryInto,
    error::Error,
    fmt::{Display, Formatter},
};

pub struct LoginId(String);

impl LoginId {
    pub fn extract(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

// login id には技術的な制限はないが、使用可能な最大文字数（バイト数ではない）は定義しておく
// ui の設定と同期させること
const LOGIN_ID_MAX_LENGTH: usize = 100;

impl TryInto<LoginId> for String {
    type Error = ConvertLoginIdError;

    fn try_into(self) -> Result<LoginId, Self::Error> {
        match self.chars().count() {
            n if n == 0 => Err(ConvertLoginIdError::Empty),
            n if n > LOGIN_ID_MAX_LENGTH => Err(ConvertLoginIdError::TooLong),
            _ => Ok(LoginId(self)),
        }
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
