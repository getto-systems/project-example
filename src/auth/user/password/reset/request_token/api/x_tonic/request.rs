use crate::auth::user::password::reset::request_token::y_protobuf::service::RequestResetTokenRequestPb;

use crate::auth::user::password::reset::request_token::infra::{
    RequestResetPasswordTokenFields, RequestResetPasswordTokenFieldsExtract,
};

use crate::auth::user::login_id::kernel::data::{LoginId, ValidateLoginIdError};

impl RequestResetPasswordTokenFieldsExtract for RequestResetTokenRequestPb {
    fn convert(self) -> Result<RequestResetPasswordTokenFields, ValidateLoginIdError> {
        Ok(RequestResetPasswordTokenFields {
            login_id: LoginId::convert(self.login_id)?,
        })
    }
}
