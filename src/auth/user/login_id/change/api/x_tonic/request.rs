use crate::auth::user::login_id::{
    change::{
        data::ValidateOverwriteLoginIdFieldsError,
        infra::{OverwriteLoginIdFields, OverwriteLoginIdFieldsExtract},
        y_protobuf::service::OverwriteLoginIdRequestPb,
    },
    kernel::data::LoginId,
};

impl OverwriteLoginIdFieldsExtract for OverwriteLoginIdRequestPb {
    fn convert(self) -> Result<OverwriteLoginIdFields, ValidateOverwriteLoginIdFieldsError> {
        Ok(OverwriteLoginIdFields {
            login_id: LoginId::convert(self.login_id)
                .map_err(ValidateOverwriteLoginIdFieldsError::InvalidCurrentLoginId)?,
            new_login_id: LoginId::convert(self.new_login_id)
                .map_err(ValidateOverwriteLoginIdFieldsError::InvalidNewLoginId)?,
        })
    }
}
