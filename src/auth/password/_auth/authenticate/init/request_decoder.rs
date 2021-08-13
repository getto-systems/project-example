use crate::auth::password::_common::y_protobuf::service::AuthenticatePasswordRequestPb;

use crate::auth::password::{
    _auth::authenticate::infra::AuthenticatePasswordRequestDecoder,
    _common::authenticate::infra::AuthenticatePasswordFieldsExtract,
};

pub struct PbAuthenticatePasswordRequestDecoder {
    request: AuthenticatePasswordRequestPb,
}

impl PbAuthenticatePasswordRequestDecoder {
    pub const fn new(request: AuthenticatePasswordRequestPb) -> Self {
        Self { request }
    }
}

impl AuthenticatePasswordRequestDecoder for PbAuthenticatePasswordRequestDecoder {
    fn decode(self) -> AuthenticatePasswordFieldsExtract {
        self.request.into()
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::{
        _auth::authenticate::infra::AuthenticatePasswordRequestDecoder,
        _common::authenticate::infra::AuthenticatePasswordFieldsExtract,
    };

    pub enum StaticAuthenticatePasswordRequestDecoder {
        Valid(AuthenticatePasswordFieldsExtract),
    }

    impl AuthenticatePasswordRequestDecoder for StaticAuthenticatePasswordRequestDecoder {
        fn decode(self) -> AuthenticatePasswordFieldsExtract {
            match self {
                Self::Valid(fields) => fields,
            }
        }
    }
}
