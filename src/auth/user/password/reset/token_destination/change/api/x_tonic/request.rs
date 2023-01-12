use crate::auth::user::password::reset::token_destination::change::y_protobuf::service::ChangeResetTokenDestinationRequestPb;

use crate::auth::user::password::reset::token_destination::change::infra::{
    ChangeResetTokenDestinationFields, ChangeResetTokenDestinationFieldsExtract,
};

use crate::auth::user::{
    login_id::kernel::data::LoginId,
    password::reset::token_destination::change::data::ValidateChangeResetTokenDestinationFieldsError,
};

impl ChangeResetTokenDestinationFieldsExtract for ChangeResetTokenDestinationRequestPb {
    fn convert(
        self,
    ) -> Result<ChangeResetTokenDestinationFields, ValidateChangeResetTokenDestinationFieldsError>
    {
        type Error = ValidateChangeResetTokenDestinationFieldsError;
        Ok(ChangeResetTokenDestinationFields {
            login_id: LoginId::convert(self.login_id).map_err(Error::InvalidLoginId)?,
            from: self.from.try_into().map_err(Error::InvalidFrom)?,
            to: self.to.try_into().map_err(Error::InvalidTo)?,
        })
    }
}
