use crate::auth::user::password::reset::request_token::infra::{
    RequestResetTokenFields, RequestResetTokenFieldsExtract,
};

use crate::auth::user::{
    login_id::kernel::data::LoginId,
    password::reset::request_token::data::ValidateRequestResetTokenFieldsError,
};

impl RequestResetTokenFields {
    pub fn validate(
        fields: RequestResetTokenFieldsExtract,
    ) -> Result<Self, ValidateRequestResetTokenFieldsError> {
        Ok(Self {
            login_id: LoginId::convert(fields.login_id)
                .map_err(ValidateRequestResetTokenFieldsError::InvalidLoginId)?,
        })
    }
}
