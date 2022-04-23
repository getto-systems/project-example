use crate::auth::user::account::unregister::y_protobuf::service::UnregisterAuthUserAccountRequestPb;

use crate::auth::user::account::unregister::infra::UnregisterAuthUserAccountRequestDecoder;

use crate::auth::user::{
    account::unregister::data::ValidateUnregisterAuthUserAccountFieldsError,
    login_id::kernel::data::LoginId,
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
    fn decode(self) -> Result<LoginId, ValidateUnregisterAuthUserAccountFieldsError> {
        Ok(LoginId::validate(self.request.login_id)
            .map_err(ValidateUnregisterAuthUserAccountFieldsError::InvalidLoginId)?)
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::account::unregister::infra::UnregisterAuthUserAccountRequestDecoder;

    use crate::auth::user::{
        account::unregister::data::ValidateUnregisterAuthUserAccountFieldsError,
        login_id::kernel::data::LoginId,
    };

    pub enum StaticUnregisterAuthUserAccountRequestDecoder {
        Valid(LoginId),
    }

    impl UnregisterAuthUserAccountRequestDecoder for StaticUnregisterAuthUserAccountRequestDecoder {
        fn decode(self) -> Result<LoginId, ValidateUnregisterAuthUserAccountFieldsError> {
            match self {
                Self::Valid(login_id) => Ok(login_id),
            }
        }
    }
}
