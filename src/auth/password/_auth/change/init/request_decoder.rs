use crate::auth::password::_common::y_protobuf::service::ChangePasswordRequestPb;

use crate::auth::password::{
    _auth::change::infra::ChangePasswordRequestDecoder,
    _common::change::infra::ChangePasswordFieldsExtract,
};

pub struct PbChangePasswordRequestDecoder {
    request: ChangePasswordRequestPb,
}

impl PbChangePasswordRequestDecoder {
    pub const fn new(request: ChangePasswordRequestPb) -> Self {
        Self { request }
    }
}

impl ChangePasswordRequestDecoder for PbChangePasswordRequestDecoder {
    fn decode(self) -> ChangePasswordFieldsExtract {
        ChangePasswordFieldsExtract {
            current_password: self.request.current_password,
            new_password: self.request.new_password,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::{
        _auth::change::infra::ChangePasswordRequestDecoder,
        _common::change::infra::ChangePasswordFieldsExtract,
    };

    pub enum StaticChangePasswordRequestDecoder {
        Valid(ChangePasswordFieldsExtract),
    }

    impl ChangePasswordRequestDecoder for StaticChangePasswordRequestDecoder {
        fn decode(self) -> ChangePasswordFieldsExtract {
            match self {
                Self::Valid(fields) => fields,
            }
        }
    }
}
