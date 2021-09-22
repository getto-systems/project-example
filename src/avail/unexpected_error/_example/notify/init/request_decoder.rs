use crate::avail::unexpected_error::_common::y_protobuf::service::NotifyRequestPb;

use crate::avail::unexpected_error::{
    _common::notify::infra::NotifyUnexpectedErrorFieldsExtract,
    _example::notify::infra::NotifyUnexpectedErrorRequestDecoder,
};

pub struct PbNotifyUnexpectedErrorRequestDecoder {
    request: NotifyRequestPb,
}

impl PbNotifyUnexpectedErrorRequestDecoder {
    pub const fn new(request: NotifyRequestPb) -> Self {
        Self { request }
    }
}

impl NotifyUnexpectedErrorRequestDecoder for PbNotifyUnexpectedErrorRequestDecoder {
    fn decode(self) -> NotifyUnexpectedErrorFieldsExtract {
        NotifyUnexpectedErrorFieldsExtract {
            err: self.request.err,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::avail::unexpected_error::{
        _common::notify::infra::NotifyUnexpectedErrorFieldsExtract,
        _example::notify::infra::NotifyUnexpectedErrorRequestDecoder,
    };

    pub struct StaticNotifyUnexpectedErrorRequestDecoder {
        pub fields: NotifyUnexpectedErrorFieldsExtract,
    }

    impl NotifyUnexpectedErrorRequestDecoder for StaticNotifyUnexpectedErrorRequestDecoder {
        fn decode(self) -> NotifyUnexpectedErrorFieldsExtract {
            self.fields
        }
    }
}
