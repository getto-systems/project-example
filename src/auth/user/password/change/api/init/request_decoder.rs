use crate::auth::user::password::change::y_protobuf::service::{
    ChangePasswordRequestPb, OverridePasswordRequestPb,
};

use crate::auth::user::password::change::infra::{
    ChangePasswordFieldsExtract, ChangePasswordRequestDecoder, OverridePasswordFieldsExtract,
    OverridePasswordRequestDecoder,
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

pub struct PbOverridePasswordRequestDecoder {
    request: OverridePasswordRequestPb,
}

impl PbOverridePasswordRequestDecoder {
    pub const fn new(request: OverridePasswordRequestPb) -> Self {
        Self { request }
    }
}

impl OverridePasswordRequestDecoder for PbOverridePasswordRequestDecoder {
    fn decode(self) -> OverridePasswordFieldsExtract {
        OverridePasswordFieldsExtract {
            login_id: self.request.login_id,
            new_password: self.request.new_password,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::change::infra::{
        ChangePasswordFieldsExtract, ChangePasswordRequestDecoder, OverridePasswordFieldsExtract,
        OverridePasswordRequestDecoder,
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

    pub enum StaticOverridePasswordRequestDecoder {
        Valid(OverridePasswordFieldsExtract),
    }

    impl OverridePasswordRequestDecoder for StaticOverridePasswordRequestDecoder {
        fn decode(self) -> OverridePasswordFieldsExtract {
            match self {
                Self::Valid(fields) => fields,
            }
        }
    }
}
