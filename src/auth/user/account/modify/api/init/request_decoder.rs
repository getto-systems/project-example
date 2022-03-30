use crate::auth::user::account::modify::y_protobuf::service::{
    ModifyAuthUserAccountDataPb, ModifyAuthUserAccountRequestPb,
};

use crate::auth::user::account::modify::infra::{
    ModifyAuthUserAccountFields, ModifyAuthUserAccountRequestDecoder,
};

use crate::auth::user::{
    account::modify::data::{
        AuthUserAccountChanges, ValidateAuthUserAccountChangesError, ValidateAuthUserAccountError,
    },
    kernel::data::GrantedAuthRoles,
    login_id::kernel::data::LoginId,
};

pub struct PbModifyAuthUserAccountRequestDecoder {
    request: ModifyAuthUserAccountRequestPb,
}

impl PbModifyAuthUserAccountRequestDecoder {
    pub const fn new(request: ModifyAuthUserAccountRequestPb) -> Self {
        Self { request }
    }
}

impl ModifyAuthUserAccountRequestDecoder for PbModifyAuthUserAccountRequestDecoder {
    fn decode(self) -> Result<ModifyAuthUserAccountFields, ValidateAuthUserAccountError> {
        Ok(ModifyAuthUserAccountFields {
            login_id: LoginId::validate(self.request.login_id)
                .map_err(ValidateAuthUserAccountError::InvalidLoginId)?,
            from: validate_data(self.request.from)
                .map_err(ValidateAuthUserAccountError::InvalidFrom)?,
            to: validate_data(self.request.to).map_err(ValidateAuthUserAccountError::InvalidTo)?,
        })
    }
}

fn validate_data(
    data: Option<ModifyAuthUserAccountDataPb>,
) -> Result<AuthUserAccountChanges, ValidateAuthUserAccountChangesError> {
    match data {
        None => Err(ValidateAuthUserAccountChangesError::NotFound),
        Some(data) => Ok(AuthUserAccountChanges {
            granted_roles: GrantedAuthRoles::validate(data.granted_roles)
                .map_err(ValidateAuthUserAccountChangesError::InvalidGrantedRoles)?,
        }),
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::account::modify::infra::{
        ModifyAuthUserAccountFields, ModifyAuthUserAccountRequestDecoder,
    };

    use crate::auth::user::account::modify::data::ValidateAuthUserAccountError;

    pub enum StaticModifyAuthUserAccountRequestDecoder {
        Valid(ModifyAuthUserAccountFields),
    }

    impl ModifyAuthUserAccountRequestDecoder for StaticModifyAuthUserAccountRequestDecoder {
        fn decode(self) -> Result<ModifyAuthUserAccountFields, ValidateAuthUserAccountError> {
            match self {
                Self::Valid(fields) => Ok(fields),
            }
        }
    }
}
