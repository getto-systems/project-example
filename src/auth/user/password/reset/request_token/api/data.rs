use crate::auth::user::login_id::kernel::data::ValidateLoginIdError;

pub enum ValidateRequestResetTokenFieldsError {
    InvalidLoginId(ValidateLoginIdError),
}

impl std::fmt::Display for ValidateRequestResetTokenFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => write!(f, "login-id: {}", err),
        }
    }
}

pub struct NotifyResetTokenResponse {
    message_id: String,
}

impl NotifyResetTokenResponse {
    pub fn new(message_id: String) -> Self {
        Self { message_id }
    }
}

impl std::fmt::Display for NotifyResetTokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "message-id: {}", self.message_id)
    }
}

pub enum EncodeResetTokenError {
    InfraError(String),
}

impl std::fmt::Display for EncodeResetTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "encode error: {}", err),
        }
    }
}

pub enum NotifyResetTokenError {
    NoDestination,
    InfraError(String),
}

impl std::fmt::Display for NotifyResetTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NoDestination => write!(f, "no reset token destination"),
            Self::InfraError(err) => write!(f, "notify reset token error; {}", err),
        }
    }
}
