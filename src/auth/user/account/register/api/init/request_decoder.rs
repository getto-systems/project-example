use crate::auth::user::account::kernel::data::{AuthUserAttributes, AuthUserAttributesExtract};
use crate::auth::user::account::register::y_protobuf::service::RegisterAuthUserAccountRequestPb;

use crate::auth::user::account::register::infra::{
    RegisterAuthUserAccountFields, RegisterAuthUserAccountRequestDecoder,
};

use crate::auth::user::password::reset::kernel::data::{
    ResetTokenDestination, ResetTokenDestinationExtract,
};
use crate::auth::user::{
    account::register::data::ValidateRegisterAuthUserAccountFieldsError,
    kernel::data::GrantedAuthRoles, login_id::kernel::data::LoginId,
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
    fn decode(
        self,
    ) -> Result<RegisterAuthUserAccountFields, ValidateRegisterAuthUserAccountFieldsError> {
        Ok(RegisterAuthUserAccountFields {
            login_id: LoginId::convert(self.request.login_id)
                .map_err(ValidateRegisterAuthUserAccountFieldsError::InvalidLoginId)?,
            granted_roles: GrantedAuthRoles::convert(self.request.granted_roles)
                .map_err(ValidateRegisterAuthUserAccountFieldsError::InvalidGrantedRoles)?,
            reset_token_destination: ResetTokenDestination::convert(
                self.request
                    .reset_token_destination
                    .and_then(|destination| match destination.r#type.as_str() {
                        "email" => Some(ResetTokenDestinationExtract::Email(destination.email)),
                        _ => None,
                    })
                    .unwrap_or(ResetTokenDestinationExtract::None),
            )
            .map_err(ValidateRegisterAuthUserAccountFieldsError::InvalidResetTokenDestination)?,
            attrs: AuthUserAttributes::convert(AuthUserAttributesExtract {
                memo: self.request.memo,
            }).map_err(ValidateRegisterAuthUserAccountFieldsError::InvalidAttrs)?,
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::account::register::infra::{
        RegisterAuthUserAccountFields, RegisterAuthUserAccountRequestDecoder,
    };

    use crate::auth::user::account::register::data::ValidateRegisterAuthUserAccountFieldsError;

    pub enum StaticRegisterAuthUserAccountRequestDecoder {
        Valid(RegisterAuthUserAccountFields),
    }

    impl RegisterAuthUserAccountRequestDecoder for StaticRegisterAuthUserAccountRequestDecoder {
        fn decode(
            self,
        ) -> Result<RegisterAuthUserAccountFields, ValidateRegisterAuthUserAccountFieldsError>
        {
            match self {
                Self::Valid(fields) => Ok(fields),
            }
        }
    }
}
