use crate::auth::user::password::remote::y_protobuf::service::AuthenticatePasswordRequestPb;

use crate::auth::user::password::remote::{
    authenticate::infra::AuthenticatePasswordRequestDecoder,
    proxy_authenticate::infra::AuthenticatePasswordFieldsExtract,
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
    use crate::auth::user::password::remote::{
        authenticate::infra::AuthenticatePasswordRequestDecoder,
        proxy_authenticate::infra::AuthenticatePasswordFieldsExtract,
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
