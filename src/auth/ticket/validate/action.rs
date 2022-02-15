use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{
    validate_auth_token, ValidateAuthTokenEvent, ValidateAuthTokenInfra,
};

use crate::auth::ticket::validate::infra::ValidateApiTokenRequestDecoder;

use crate::auth::{ticket::kernel::data::ValidateAuthRolesError, user::kernel::data::AuthUser};

pub enum ValidateApiTokenState {
    Validate(ValidateAuthTokenEvent),
    PermissionError(ValidateAuthRolesError),
    Success(AuthUser),
}

impl std::fmt::Display for ValidateApiTokenState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => event.fmt(f),
            Self::PermissionError(err) => err.fmt(f),
            Self::Success(user) => write!(f, "validate api token success; {}", user),
        }
    }
}

pub struct ValidateApiTokenAction<R: ValidateApiTokenRequestDecoder, M: ValidateAuthTokenInfra> {
    pubsub: ActionStatePubSub<ValidateApiTokenState>,
    request_decoder: R,
    material: M,
}

impl<R: ValidateApiTokenRequestDecoder, M: ValidateAuthTokenInfra> ValidateApiTokenAction<R, M> {
    pub fn with_material(request_decoder: R, material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            request_decoder,
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&ValidateApiTokenState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<ValidateApiTokenState> {
        let pubsub = self.pubsub;

        let require_roles = self.request_decoder.decode();

        let ticket = validate_auth_token(&self.material, |event| {
            pubsub.post(ValidateApiTokenState::Validate(event))
        })
        .await?;

        let ticket = ticket
            .check_enough_permission(require_roles)
            .map_err(|err| pubsub.post(ValidateApiTokenState::PermissionError(err)))?;

        Ok(pubsub.post(ValidateApiTokenState::Success(ticket.into_user())))
    }
}
