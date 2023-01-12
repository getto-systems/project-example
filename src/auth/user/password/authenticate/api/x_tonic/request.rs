use crate::auth::user::password::authenticate::y_protobuf::service::AuthenticateWithPasswordRequestPb;

use crate::auth::user::password::{
    authenticate::infra::{AuthenticateWithPasswordFields, AuthenticateWithPasswordFieldsExtract},
    kernel::infra::PlainPassword,
};

use crate::auth::user::{
    login_id::kernel::data::LoginId,
    password::authenticate::data::ValidateAuthenticateWithPasswordFieldsError,
};

impl AuthenticateWithPasswordFieldsExtract for AuthenticateWithPasswordRequestPb {
    fn convert(
        self,
    ) -> Result<AuthenticateWithPasswordFields, ValidateAuthenticateWithPasswordFieldsError> {
        Ok(AuthenticateWithPasswordFields {
            login_id: LoginId::convert(self.login_id)
                .map_err(ValidateAuthenticateWithPasswordFieldsError::InvalidLoginId)?,
            plain_password: PlainPassword::convert(self.password)
                .map_err(ValidateAuthenticateWithPasswordFieldsError::InvalidPassword)?,
        })
    }
}
