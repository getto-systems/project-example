use crate::avail::unexpected_error::notify::y_protobuf::service::NotifyRequestPb;

use crate::avail::unexpected_error::notify::infra::{
    NotifyUnexpectedErrorFields, NotifyUnexpectedErrorFieldsExtract,
};

impl NotifyUnexpectedErrorFieldsExtract for NotifyRequestPb {
    fn convert(self) -> NotifyUnexpectedErrorFields {
        NotifyUnexpectedErrorFields { err: self.err }
    }
}
