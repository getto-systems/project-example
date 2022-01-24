use crate::auth::user::password::reset::remote::y_protobuf::service::ResetPasswordRequestPb;

use crate::auth::user::password::reset::remote::reset::infra::{
    ResetPasswordFieldsExtract, ResetPasswordRequestDecoder,
};

pub struct PbResetPasswordRequestDecoder {
    request: ResetPasswordRequestPb,
}

impl PbResetPasswordRequestDecoder {
    pub const fn new(request: ResetPasswordRequestPb) -> Self {
        Self { request }
    }
}

impl ResetPasswordRequestDecoder for PbResetPasswordRequestDecoder {
    fn decode(self) -> ResetPasswordFieldsExtract {
        ResetPasswordFieldsExtract {
            reset_token: self.request.reset_token,
            login_id: self.request.login_id,
            password: self.request.password,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::reset::remote::reset::infra::{
        ResetPasswordFieldsExtract, ResetPasswordRequestDecoder,
    };

    pub struct StaticResetPasswordRequestDecoder {
        pub fields: ResetPasswordFieldsExtract,
    }

    impl ResetPasswordRequestDecoder for StaticResetPasswordRequestDecoder {
        fn decode(self) -> ResetPasswordFieldsExtract {
            self.fields
        }
    }
}
