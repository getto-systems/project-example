use crate::auth::{
    auth_ticket::_auth::kernel::infra::CheckAuthNonceInfra,
    auth_user::_auth::kernel::infra::AuthUserInfra,
    password::_auth::kernel::infra::AuthUserPasswordMatchInfra,
};

pub trait AuthenticatePasswordInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type UserInfra: AuthUserInfra;
    type PasswordInfra: AuthUserPasswordMatchInfra;
    type RequestDecoder: AuthenticatePasswordRequestDecoder;

    fn extract(
        self,
    ) -> (
        Self::CheckNonceInfra,
        Self::UserInfra,
        Self::PasswordInfra,
        Self::RequestDecoder,
    );
}

pub trait AuthenticatePasswordRequestDecoder {
    fn decode(self) -> AuthenticatePasswordFieldsExtract;
}

pub struct AuthenticatePasswordFieldsExtract {
    pub login_id: String,
    pub password: String,
}
