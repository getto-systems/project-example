use crate::auth::ticket::validate::y_protobuf::service::ValidateApiTokenRequestPb;

use crate::auth::ticket::validate::infra::ValidateApiTokenRequestDecoder;

use crate::auth::user::kernel::data::RequireAuthRoles;

pub struct PbValidateApiTokenRequestDecoder {
    request: ValidateApiTokenRequestPb,
}

impl PbValidateApiTokenRequestDecoder {
    pub const fn new(request: ValidateApiTokenRequestPb) -> Self {
        Self { request }
    }
}

impl ValidateApiTokenRequestDecoder for PbValidateApiTokenRequestDecoder {
    fn decode(self) -> RequireAuthRoles {
        if self.request.allow_any_role {
            RequireAuthRoles::Nothing
        } else {
            RequireAuthRoles::restore_has_any(
                self.request
                    .require_roles
                    .iter()
                    .map(|role| role.as_str())
                    .collect(),
            )
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::{
        ticket::validate::infra::ValidateApiTokenRequestDecoder,
        user::kernel::data::RequireAuthRoles,
    };

    pub struct StaticValidateApiTokenRequestDecoder {
        pub require_roles: RequireAuthRoles,
    }

    impl ValidateApiTokenRequestDecoder for StaticValidateApiTokenRequestDecoder {
        fn decode(self) -> RequireAuthRoles {
            self.require_roles
        }
    }
}
