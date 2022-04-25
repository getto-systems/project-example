use crate::auth::user::account::unregister::y_protobuf::service::UnregisterAuthUserAccountRequestPb;

use crate::auth::user::account::unregister::infra::{
    UnregisterAuthUserAccountFieldsExtract, UnregisterAuthUserAccountRequestDecoder,
};

pub struct PbUnregisterAuthUserAccountRequestDecoder {
    request: UnregisterAuthUserAccountRequestPb,
}

impl PbUnregisterAuthUserAccountRequestDecoder {
    pub const fn new(request: UnregisterAuthUserAccountRequestPb) -> Self {
        Self { request }
    }
}

impl UnregisterAuthUserAccountRequestDecoder for PbUnregisterAuthUserAccountRequestDecoder {
    fn decode(self) -> UnregisterAuthUserAccountFieldsExtract {
        UnregisterAuthUserAccountFieldsExtract {
            login_id: self.request.login_id,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::account::unregister::infra::{
        UnregisterAuthUserAccountFieldsExtract, UnregisterAuthUserAccountRequestDecoder,
    };

    pub enum StaticUnregisterAuthUserAccountRequestDecoder {
        Valid(UnregisterAuthUserAccountFieldsExtract),
    }

    impl UnregisterAuthUserAccountRequestDecoder for StaticUnregisterAuthUserAccountRequestDecoder {
        fn decode(self) -> UnregisterAuthUserAccountFieldsExtract {
            match self {
                Self::Valid(fields) => fields,
            }
        }
    }
}
