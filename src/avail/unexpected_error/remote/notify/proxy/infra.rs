use crate::avail::unexpected_error::remote::notify::infra::NotifyUnexpectedErrorFieldsExtract;

use crate::z_lib::remote::message::data::MessageError;

pub trait NotifyUnexpectedErrorProxyRequestDecoder {
    fn decode(self) -> Result<NotifyUnexpectedErrorFieldsExtract, MessageError>;
}
