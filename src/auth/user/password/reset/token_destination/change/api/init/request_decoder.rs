use crate::auth::user::password::reset::token_destination::{
    change::y_protobuf::service::ChangeResetTokenDestinationRequestPb,
    kernel::y_protobuf::service::ResetTokenDestinationPb,
};

use crate::auth::user::password::reset::token_destination::change::infra::{
    ChangeResetTokenDestinationFields, ChangeResetTokenDestinationRequestDecoder,
};

use crate::auth::user::{
    login_id::kernel::data::LoginId,
    password::reset::{
        kernel::data::{ResetTokenDestination, ResetTokenDestinationExtract},
        token_destination::change::data::{
            ValidateChangeResetTokenDestinationChangesError,
            ValidateChangeResetTokenDestinationFieldsError,
        },
    },
};

pub struct PbChangeResetTokenDestinationRequestDecoder {
    request: ChangeResetTokenDestinationRequestPb,
}

impl PbChangeResetTokenDestinationRequestDecoder {
    pub const fn new(request: ChangeResetTokenDestinationRequestPb) -> Self {
        Self { request }
    }
}

impl ChangeResetTokenDestinationRequestDecoder for PbChangeResetTokenDestinationRequestDecoder {
    fn decode(
        self,
    ) -> Result<ChangeResetTokenDestinationFields, ValidateChangeResetTokenDestinationFieldsError>
    {
        Ok(ChangeResetTokenDestinationFields {
            login_id: LoginId::convert(self.request.login_id)
                .map_err(ValidateChangeResetTokenDestinationFieldsError::InvalidLoginId)?,
            from: validate_data(self.request.from)
                .map_err(ValidateChangeResetTokenDestinationFieldsError::InvalidFrom)?,
            to: validate_data(self.request.to)
                .map_err(ValidateChangeResetTokenDestinationFieldsError::InvalidTo)?,
        })
    }
}

fn validate_data(
    data: Option<ResetTokenDestinationPb>,
) -> Result<ResetTokenDestination, ValidateChangeResetTokenDestinationChangesError> {
    match data {
        None => Err(ValidateChangeResetTokenDestinationChangesError::NotFound),
        Some(destination) => ResetTokenDestination::convert({
            if destination.r#type == "email" {
                ResetTokenDestinationExtract::Email(destination.email)
            } else {
                ResetTokenDestinationExtract::None
            }
        })
        .map_err(ValidateChangeResetTokenDestinationChangesError::InvalidResetTokenDestination),
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::reset::token_destination::change::{
        data::ValidateChangeResetTokenDestinationFieldsError,
        infra::{ChangeResetTokenDestinationFields, ChangeResetTokenDestinationRequestDecoder},
    };

    pub enum StaticChangeResetTokenDestinationRequestDecoder {
        Valid(ChangeResetTokenDestinationFields),
    }

    impl ChangeResetTokenDestinationRequestDecoder for StaticChangeResetTokenDestinationRequestDecoder {
        fn decode(
            self,
        ) -> Result<ChangeResetTokenDestinationFields, ValidateChangeResetTokenDestinationFieldsError>
        {
            match self {
                Self::Valid(fields) => Ok(fields),
            }
        }
    }
}
