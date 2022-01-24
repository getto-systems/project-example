use crate::auth::user::password::{
    remote::kernel::data::ResetTokenDestination,
    reset::remote::reset::data::{NotifyResetPasswordError, NotifyResetPasswordResponse},
};

use crate::auth::user::password::{
    remote::kernel::data::ResetToken,
    reset::remote::{kernel::data::ResetTokenEncoded, reset::data::DecodeResetTokenError},
};

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

#[async_trait::async_trait]
pub trait ResetPasswordNotifier {
    async fn notify(
        &self,
        destination: ResetTokenDestination,
    ) -> Result<NotifyResetPasswordResponse, NotifyResetPasswordError>;
}
