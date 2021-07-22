use crate::auth::{
    auth_ticket::_auth::kernel::infra::{AuthClockInfra, CheckAuthNonceInfra},
    auth_user::_auth::kernel::infra::AuthUserInfra,
    password::_auth::kernel::infra::AuthUserPasswordHashInfra,
};

use crate::auth::password::reset::_auth::reset::event::ResetPasswordEvent;

use crate::auth::password::{
    _auth::kernel::data::{PasswordHashRepositoryError, ResetToken},
    reset::_auth::{kernel::data::ResetTokenEncoded, reset::data::DecodeResetTokenError},
};

pub trait ResetPasswordInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type ClockInfra: AuthClockInfra;
    type UserInfra: AuthUserInfra;
    type PasswordInfra: AuthUserPasswordHashInfra;
    type RequestDecoder: ResetPasswordRequestDecoder;
    type TokenDecoder: ResetTokenDecoder;

    fn extract(
        self,
    ) -> (
        Self::CheckNonceInfra,
        Self::ClockInfra,
        Self::UserInfra,
        Self::PasswordInfra,
        Self::RequestDecoder,
        Self::TokenDecoder,
    );
}

pub trait ResetTokenDecoder {
    fn decode(&self, token: &ResetTokenEncoded) -> Result<ResetToken, DecodeResetTokenError>;
}

pub trait ResetPasswordRequestDecoder {
    fn decode(self) -> ResetPasswordFieldsExtract;
}

pub struct ResetPasswordFieldsExtract {
    pub reset_token: String,
    pub login_id: String,
    pub password: String,
}

impl Into<ResetPasswordEvent> for PasswordHashRepositoryError {
    fn into(self) -> ResetPasswordEvent {
        match self {
            Self::RepositoryError(err) => ResetPasswordEvent::RepositoryError(err),
            Self::PasswordHashError(err) => ResetPasswordEvent::PasswordHashError(err),
        }
    }
}
