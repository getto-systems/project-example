use std::collections::HashSet;

use crate::auth::ticket::remote::y_protobuf::service::ValidateApiTokenRequestPb;

use crate::auth::ticket::validate::infra::ValidateApiTokenRequestDecoder;

use crate::auth::user::remote::kernel::data::RequireAuthRoles;

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
            let mut require_roles = HashSet::new();
            self.request.require_roles.into_iter().for_each(|role| {
                require_roles.insert(role);
            });
            RequireAuthRoles::HasAny(require_roles)
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::{
        ticket::validate::infra::ValidateApiTokenRequestDecoder,
        user::remote::kernel::data::RequireAuthRoles,
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
