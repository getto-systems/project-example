use crate::auth::user::account::unregister::y_protobuf::service::UnregisterAuthUserAccountRequestPb;

use crate::auth::user::account::unregister::infra::{
    UnregisterAuthUserAccountFields, UnregisterAuthUserAccountFieldsExtract,
};

use crate::auth::user::login_id::kernel::data::{LoginId, ValidateLoginIdError};

impl UnregisterAuthUserAccountFieldsExtract for UnregisterAuthUserAccountRequestPb {
    fn convert(self) -> Result<UnregisterAuthUserAccountFields, ValidateLoginIdError> {
        Ok(UnregisterAuthUserAccountFields {
            login_id: LoginId::convert(self.login_id)?,
        })
    }
}
