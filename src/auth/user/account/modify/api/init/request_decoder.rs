use crate::auth::user::account::modify::y_protobuf::service::{
    ModifyAuthUserAccountChangesPb, ModifyAuthUserAccountRequestPb,
};

use crate::auth::user::account::modify::infra::{
    ModifyAuthUserAccountFields, ModifyAuthUserAccountRequestDecoder,
};

use crate::auth::user::{
    account::modify::data::{
        ModifyAuthUserAccountChanges, ValidateModifyAuthUserAccountChangesError, ValidateModifyAuthUserAccountFieldsError,
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
    fn decode(self) -> Result<ModifyAuthUserAccountFields, ValidateModifyAuthUserAccountFieldsError> {
        Ok(ModifyAuthUserAccountFields {
            login_id: LoginId::convert(self.request.login_id)
                .map_err(ValidateModifyAuthUserAccountFieldsError::InvalidLoginId)?,
            from: validate_data(self.request.from)
                .map_err(ValidateModifyAuthUserAccountFieldsError::InvalidFrom)?,
            to: validate_data(self.request.to).map_err(ValidateModifyAuthUserAccountFieldsError::InvalidTo)?,
        })
    }
}

fn validate_data(
    data: Option<ModifyAuthUserAccountChangesPb>,
) -> Result<ModifyAuthUserAccountChanges, ValidateModifyAuthUserAccountChangesError> {
    match data {
        None => Err(ValidateModifyAuthUserAccountChangesError::NotFound),
        Some(data) => Ok(ModifyAuthUserAccountChanges {
            granted_roles: GrantedAuthRoles::validate(data.granted_roles)
                .map_err(ValidateModifyAuthUserAccountChangesError::InvalidGrantedRoles)?,
        }),
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::account::modify::infra::{
        ModifyAuthUserAccountFields, ModifyAuthUserAccountRequestDecoder,
    };

    use crate::auth::user::account::modify::data::ValidateModifyAuthUserAccountFieldsError;

    pub enum StaticModifyAuthUserAccountRequestDecoder {
        Valid(ModifyAuthUserAccountFields),
    }

    impl ModifyAuthUserAccountRequestDecoder for StaticModifyAuthUserAccountRequestDecoder {
        fn decode(self) -> Result<ModifyAuthUserAccountFields, ValidateModifyAuthUserAccountFieldsError> {
            match self {
                Self::Valid(fields) => Ok(fields),
            }
        }
    }
}
