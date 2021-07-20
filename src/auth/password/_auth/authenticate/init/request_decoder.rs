use crate::auth::password::_common::y_protobuf::service::AuthenticatePasswordRequestPb;

use crate::auth::password::_auth::authenticate::infra::{
    AuthenticatePasswordFieldsExtract, AuthenticatePasswordRequestDecoder,
};

pub struct TonicAuthenticatePasswordRequestDecoder {
    request: AuthenticatePasswordRequestPb,
}

impl TonicAuthenticatePasswordRequestDecoder {
    pub const fn new(request: AuthenticatePasswordRequestPb) -> Self {
        Self { request }
    }
}

impl AuthenticatePasswordRequestDecoder for TonicAuthenticatePasswordRequestDecoder {
    fn decode(self) -> AuthenticatePasswordFieldsExtract {
        self.request.into()
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::_auth::authenticate::infra::{
        AuthenticatePasswordFieldsExtract, AuthenticatePasswordRequestDecoder,
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
