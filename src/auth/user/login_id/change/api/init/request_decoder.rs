use crate::auth::user::login_id::change::y_protobuf::service::OverwriteLoginIdRequestPb;

use crate::auth::user::login_id::change::infra::{
    OverwriteLoginIdFieldsExtract, OverwriteLoginIdRequestDecoder,
};

pub struct PbOverwriteLoginIdRequestDecoder {
    request: OverwriteLoginIdRequestPb,
}

impl PbOverwriteLoginIdRequestDecoder {
    pub const fn new(request: OverwriteLoginIdRequestPb) -> Self {
        Self { request }
    }
}

impl OverwriteLoginIdRequestDecoder for PbOverwriteLoginIdRequestDecoder {
    fn decode(self) -> OverwriteLoginIdFieldsExtract {
        OverwriteLoginIdFieldsExtract {
            login_id: self.request.login_id,
            new_login_id: self.request.new_login_id,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::login_id::change::infra::{
        OverwriteLoginIdFieldsExtract, OverwriteLoginIdRequestDecoder,
    };

    pub enum StaticOverwriteLoginIdRequestDecoder {
        Valid(OverwriteLoginIdFieldsExtract),
    }

    impl OverwriteLoginIdRequestDecoder for StaticOverwriteLoginIdRequestDecoder {
        fn decode(self) -> OverwriteLoginIdFieldsExtract {
            match self {
                Self::Valid(fields) => fields,
            }
        }
    }
}
