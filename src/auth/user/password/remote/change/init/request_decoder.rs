use crate::auth::user::password::remote::y_protobuf::service::ChangePasswordRequestPb;

use crate::auth::user::password::remote::{
    change::infra::ChangePasswordRequestDecoder, proxy_change::infra::ChangePasswordFieldsExtract,
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
    use crate::auth::user::password::remote::{
        change::infra::ChangePasswordRequestDecoder,
        proxy_change::infra::ChangePasswordFieldsExtract,
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
