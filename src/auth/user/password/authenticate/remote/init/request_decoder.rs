use crate::auth::user::password::y_protobuf::service::AuthenticatePasswordRequestPb;

use crate::auth::user::password::authenticate::remote::infra::{
    AuthenticatePasswordFieldsExtract, AuthenticatePasswordRequestDecoder,
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
        AuthenticatePasswordFieldsExtract {
            login_id: self.request.login_id,
            password: self.request.password,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::authenticate::remote::infra::{
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
