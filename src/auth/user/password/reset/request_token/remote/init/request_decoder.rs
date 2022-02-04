use crate::auth::user::password::reset::y_protobuf::service::RequestResetTokenRequestPb;

use crate::auth::user::password::reset::request_token::remote::infra::{
    RequestResetTokenFieldsExtract, RequestResetTokenRequestDecoder,
};

pub struct PbRequestResetTokenRequestDecoder {
    request: RequestResetTokenRequestPb,
}

impl PbRequestResetTokenRequestDecoder {
    pub const fn new(request: RequestResetTokenRequestPb) -> Self {
        Self { request }
    }
}

impl RequestResetTokenRequestDecoder for PbRequestResetTokenRequestDecoder {
    fn decode(self) -> RequestResetTokenFieldsExtract {
        RequestResetTokenFieldsExtract {
            login_id: self.request.login_id,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::reset::request_token::remote::infra::{
        RequestResetTokenFieldsExtract, RequestResetTokenRequestDecoder,
    };

    pub struct StaticRequestResetTokenRequestDecoder {
        pub fields: RequestResetTokenFieldsExtract,
    }

    impl RequestResetTokenRequestDecoder for StaticRequestResetTokenRequestDecoder {
        fn decode(self) -> RequestResetTokenFieldsExtract {
            self.fields
        }
    }
}
