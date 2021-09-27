use crate::{
    auth::remote::infra::ValidateApiTokenInfra,
    avail::unexpected_error::_common::notify::infra::NotifyUnexpectedErrorFieldsExtract,
};

pub trait NotifyUnexpectedErrorInfra {
    type ValidateInfra: ValidateApiTokenInfra;

    fn validate_infra(&self) -> &Self::ValidateInfra;
}

pub trait NotifyUnexpectedErrorRequestDecoder {
    fn decode(self) -> NotifyUnexpectedErrorFieldsExtract;
}
