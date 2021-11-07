use crate::avail::unexpected_error::remote::y_protobuf::service::NotifyRequestPb;

use crate::avail::unexpected_error::remote::{
    notify::infra::NotifyUnexpectedErrorRequestDecoder,
    proxy_notify::infra::NotifyUnexpectedErrorFieldsExtract,
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
    use crate::avail::unexpected_error::remote::{
        notify::infra::NotifyUnexpectedErrorRequestDecoder,
        proxy_notify::infra::NotifyUnexpectedErrorFieldsExtract,
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
