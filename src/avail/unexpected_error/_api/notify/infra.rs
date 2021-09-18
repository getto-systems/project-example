use crate::auth::_common::infra::ValidateApiTokenInfra;

use crate::z_details::_api::message::data::MessageError;

pub trait NotifyUnexpectedErrorInfra {
    type ValidateInfra: ValidateApiTokenInfra;

    fn validate_infra(&self) -> &Self::ValidateInfra;
}

pub trait NotifyUnexpectedErrorRequestDecoder {
    fn decode(self) -> Result<String, MessageError>;
}
