use crate::z_lib::remote::message::data::MessageError;

pub struct AuthProxyResponse(String);

impl AuthProxyResponse {
    pub const fn new(response: String) -> Self {
        Self(response)
    }

    pub(in crate::auth) fn extract(self) -> String {
        self.0
    }
}

pub enum AuthProxyError {
    InvalidArgument(String),
    AlreadyExists(String),
    Unauthenticated(String),
    PermissionDenied(String),
    Internal(String),
    Cancelled(String),
    InfraError(String),
    MessageError(MessageError),
}

impl std::fmt::Display for AuthProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArgument(err) => write!(f, "invalid argument; {}", err),
            Self::AlreadyExists(err) => write!(f, "already exists; {}", err),
            Self::Unauthenticated(err) => write!(f, "unauthenticated; {}", err),
            Self::PermissionDenied(err) => write!(f, "permission denied; {}", err),
            Self::Internal(err) => write!(f, "internal; {}", err),
            Self::Cancelled(err) => write!(f, "cancelled; {}", err),
            Self::InfraError(err) => write!(f, "infra error; {}", err),
            Self::MessageError(err) => write!(f, "message error; {}", err),
        }
    }
}
