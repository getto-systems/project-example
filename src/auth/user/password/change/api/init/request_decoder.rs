use crate::auth::user::password::change::y_protobuf::service::{
    ChangePasswordRequestPb, OverwritePasswordRequestPb,
};

use crate::auth::user::password::change::infra::{
    ChangePasswordFieldsExtract, ChangePasswordRequestDecoder, OverwritePasswordFieldsExtract,
    OverwritePasswordRequestDecoder,
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

pub struct PbOverwritePasswordRequestDecoder {
    request: OverwritePasswordRequestPb,
}

impl PbOverwritePasswordRequestDecoder {
    pub const fn new(request: OverwritePasswordRequestPb) -> Self {
        Self { request }
    }
}

impl OverwritePasswordRequestDecoder for PbOverwritePasswordRequestDecoder {
    fn decode(self) -> OverwritePasswordFieldsExtract {
        OverwritePasswordFieldsExtract {
            login_id: self.request.login_id,
            new_password: self.request.new_password,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::change::infra::{
        ChangePasswordFieldsExtract, ChangePasswordRequestDecoder, OverwritePasswordFieldsExtract,
        OverwritePasswordRequestDecoder,
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

    pub enum StaticOverwritePasswordRequestDecoder {
        Valid(OverwritePasswordFieldsExtract),
    }

    impl OverwritePasswordRequestDecoder for StaticOverwritePasswordRequestDecoder {
        fn decode(self) -> OverwritePasswordFieldsExtract {
            match self {
                Self::Valid(fields) => fields,
            }
        }
    }
}
