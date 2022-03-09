use crate::auth::user::login_id::change::y_protobuf::service::OverrideLoginIdRequestPb;

use crate::auth::user::login_id::change::infra::{
    OverrideLoginIdFieldsExtract, OverrideLoginIdRequestDecoder,
};

pub struct PbOverrideLoginIdRequestDecoder {
    request: OverrideLoginIdRequestPb,
}

impl PbOverrideLoginIdRequestDecoder {
    pub const fn new(request: OverrideLoginIdRequestPb) -> Self {
        Self { request }
    }
}

impl OverrideLoginIdRequestDecoder for PbOverrideLoginIdRequestDecoder {
    fn decode(self) -> OverrideLoginIdFieldsExtract {
        OverrideLoginIdFieldsExtract {
            login_id: self.request.login_id,
            new_login_id: self.request.new_login_id,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::login_id::change::infra::{
        OverrideLoginIdFieldsExtract, OverrideLoginIdRequestDecoder,
    };

    pub enum StaticOverrideLoginIdRequestDecoder {
        Valid(OverrideLoginIdFieldsExtract),
    }

    impl OverrideLoginIdRequestDecoder for StaticOverrideLoginIdRequestDecoder {
        fn decode(self) -> OverrideLoginIdFieldsExtract {
            match self {
                Self::Valid(fields) => fields,
            }
        }
    }
}
