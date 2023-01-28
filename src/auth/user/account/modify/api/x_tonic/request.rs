use crate::auth::user::account::modify::y_protobuf::service::ModifyAuthUserAccountRequestPb;

use crate::auth::user::account::modify::infra::{
    ModifyAuthUserAccountFields, ModifyAuthUserAccountFieldsExtract,
};

use crate::auth::user::{
    account::modify::data::ValidateModifyAuthUserAccountFieldsError,
    login_id::kernel::data::LoginId,
};

impl ModifyAuthUserAccountFieldsExtract for ModifyAuthUserAccountRequestPb {
    fn convert(
        self,
    ) -> Result<ModifyAuthUserAccountFields, ValidateModifyAuthUserAccountFieldsError> {
        Ok(ModifyAuthUserAccountFields {
            login_id: LoginId::convert(self.login_id)
                .map_err(ValidateModifyAuthUserAccountFieldsError::InvalidLoginId)?,
            from: self
                .from
                .try_into()
                .map_err(ValidateModifyAuthUserAccountFieldsError::InvalidFrom)?,
            to: self
                .to
                .try_into()
                .map_err(ValidateModifyAuthUserAccountFieldsError::InvalidTo)?,
        })
    }
}
