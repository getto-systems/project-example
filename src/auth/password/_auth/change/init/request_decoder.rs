use crate::auth::auth_user::_common::kernel::data::AuthUserId;
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
    fn decode(self) -> (AuthUserId, ChangePasswordFieldsExtract) {
        self.request.into()
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::{
        auth_user::_common::kernel::data::AuthUserId,
        password::{
            _auth::change::infra::ChangePasswordRequestDecoder,
            _common::change::infra::ChangePasswordFieldsExtract,
        },
    };

    pub enum StaticChangePasswordRequestDecoder {
        Valid(AuthUserId, ChangePasswordFieldsExtract),
    }

    impl ChangePasswordRequestDecoder for StaticChangePasswordRequestDecoder {
        fn decode(self) -> (AuthUserId, ChangePasswordFieldsExtract) {
            match self {
                Self::Valid(user_id, fields) => (user_id, fields),
            }
        }
    }
}
