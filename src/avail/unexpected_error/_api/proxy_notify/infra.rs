use crate::avail::unexpected_error::_common::notify::infra::NotifyUnexpectedErrorFieldsExtract;

use crate::z_details::_api::message::data::MessageError;

pub trait NotifyUnexpectedErrorProxyRequestDecoder {
    fn decode(self) -> Result<NotifyUnexpectedErrorFieldsExtract, MessageError>;
}
