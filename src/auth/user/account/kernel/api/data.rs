use crate::{
    auth::{
        ticket::kernel::data::{AuthPermissionGranted, ValidateAuthPermissionGrantedError},
        user::{
            login_id::kernel::data::{LoginId, ValidateLoginIdError},
            password::reset::kernel::data::{
                ResetPasswordTokenDestination, ValidateResetPasswordTokenDestinationError,
            },
        },
    },
    common::api::validate::data::ValidateTextError,
};

pub struct AuthUserAccount {
    pub login_id: LoginId,
    pub attrs: AuthUserAccountAttrs,
    pub reset_token_destination: ResetPasswordTokenDestination,
}

#[derive(Clone, PartialEq, Eq)]
pub struct AuthUserAccountAttrs {
    pub granted: AuthPermissionGranted,
    pub memo: AuthUserMemo,
}

impl std::fmt::Display for AuthUserAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}([{}]/{})",
            self.login_id, self.attrs.granted, self.reset_token_destination,
        )
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct AuthUserMemo(String);

impl AuthUserMemo {
    pub fn convert(value: impl AuthUserMemoExtract) -> Result<Self, ValidateAuthUserAccountError> {
        Ok(Self(
            value
                .convert()
                .map_err(ValidateAuthUserAccountError::Memo)?,
        ))
    }

    pub(in crate::auth) const fn restore(value: String) -> Self {
        Self(value)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

impl std::fmt::Display for AuthUserMemo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "memo: {}", self.0)
    }
}

pub trait AuthUserMemoExtract {
    fn convert(self) -> Result<String, ValidateTextError>;
}

#[derive(Debug)]
pub enum ValidateAuthUserAccountError {
    NotFound,
    LoginId(ValidateLoginIdError),
    ResetTokenDestination(ValidateResetPasswordTokenDestinationError),
    Granted(ValidateAuthPermissionGrantedError),
    Memo(ValidateTextError),
}

impl std::fmt::Display for ValidateAuthUserAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotFound => write!(f, "data not found"),
            Self::LoginId(err) => err.fmt(f),
            Self::ResetTokenDestination(err) => err.fmt(f),
            Self::Granted(err) => err.fmt(f),
            Self::Memo(err) => write!(f, "memo: {}", err),
        }
    }
}
