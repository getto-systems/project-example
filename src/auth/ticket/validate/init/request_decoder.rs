use crate::auth::ticket::validate::y_protobuf::service::AuthorizeRequestPb;

use crate::auth::ticket::validate::infra::AuthorizeRequestDecoder;

use crate::auth::user::kernel::data::RequireAuthRoles;

pub struct PbAuthorizeRequestDecoder {
    request: AuthorizeRequestPb,
}

impl PbAuthorizeRequestDecoder {
    pub const fn new(request: AuthorizeRequestPb) -> Self {
        Self { request }
    }
}

impl AuthorizeRequestDecoder for PbAuthorizeRequestDecoder {
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
        ticket::validate::infra::AuthorizeRequestDecoder,
        user::kernel::data::RequireAuthRoles,
    };

    pub struct StaticValidateApiTokenRequestDecoder {
        pub require_roles: RequireAuthRoles,
    }

    impl AuthorizeRequestDecoder for StaticValidateApiTokenRequestDecoder {
        fn decode(self) -> RequireAuthRoles {
            self.require_roles
        }
    }
}
