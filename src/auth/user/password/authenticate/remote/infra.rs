use crate::auth::user::{
    login_id::kernel::data::LoginId,
    password::{
        authenticate::remote::data::VerifyPasswordRepositoryError,
        kernel::infra::AuthUserPasswordMatcher,
    },
    remote::kernel::data::AuthUserId,
};

pub trait AuthenticatePasswordRequestDecoder {
    fn decode(self) -> AuthenticatePasswordFieldsExtract;
}

pub struct AuthenticatePasswordFieldsExtract {
    pub login_id: String,
    pub password: String,
}

#[async_trait::async_trait]
pub trait VerifyPasswordRepository {
    async fn verify_password<'a>(
        &self,
        login_id: &'a LoginId,
        matcher: impl AuthUserPasswordMatcher + 'a,
    ) -> Result<AuthUserId, VerifyPasswordRepositoryError>;
}
