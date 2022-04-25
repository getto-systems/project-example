use crate::auth::user::account::register::y_protobuf::service::RegisterAuthUserAccountRequestPb;

use crate::auth::user::account::register::infra::{
    RegisterAuthUserAccountFieldsExtract, RegisterAuthUserAccountRequestDecoder,
};

use crate::auth::user::{
    account::kernel::data::AuthUserAttributesExtract,
    password::reset::kernel::data::ResetTokenDestinationExtract,
};

pub struct PbRegisterAuthUserAccountRequestDecoder {
    request: RegisterAuthUserAccountRequestPb,
}

impl PbRegisterAuthUserAccountRequestDecoder {
    pub const fn new(request: RegisterAuthUserAccountRequestPb) -> Self {
        Self { request }
    }
}

impl RegisterAuthUserAccountRequestDecoder for PbRegisterAuthUserAccountRequestDecoder {
    fn decode(self) -> RegisterAuthUserAccountFieldsExtract {
        RegisterAuthUserAccountFieldsExtract {
            login_id: self.request.login_id,
            granted_roles: self.request.granted_roles,
            reset_token_destination: self
                .request
                .reset_token_destination
                .and_then(|destination| match destination.r#type.as_str() {
                    "email" => Some(ResetTokenDestinationExtract::Email(destination.email)),
                    _ => None,
                })
                .unwrap_or(ResetTokenDestinationExtract::None),
            attrs: AuthUserAttributesExtract {
                memo: self.request.memo,
            },
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::account::register::infra::{
        RegisterAuthUserAccountFieldsExtract, RegisterAuthUserAccountRequestDecoder,
    };

    pub enum StaticRegisterAuthUserAccountRequestDecoder {
        Valid(RegisterAuthUserAccountFieldsExtract),
    }

    impl RegisterAuthUserAccountRequestDecoder for StaticRegisterAuthUserAccountRequestDecoder {
        fn decode(self) -> RegisterAuthUserAccountFieldsExtract {
            match self {
                Self::Valid(fields) => fields,
            }
        }
    }
}
