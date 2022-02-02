use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::{
    ticket::remote::validate::{
        infra::ValidateApiTokenRequestDecoder,
        method::{validate_auth_token, ValidateAuthTokenEvent, ValidateAuthTokenInfra},
    },
    user::remote::kernel::data::AuthUser,
};

pub enum ValidateApiTokenState {
    Validate(ValidateAuthTokenEvent),
    Success(AuthUser),
}

impl std::fmt::Display for ValidateApiTokenState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => event.fmt(f),
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
        let p = self.pubsub;
        let m = self.material;

        let require_roles = self.request_decoder.decode();

        let ticket = validate_auth_token(&m, require_roles, |event| {
            p.post(ValidateApiTokenState::Validate(event))
        })
        .await?;

        Ok(p.post(ValidateApiTokenState::Success(ticket.into_user())))
    }
}
