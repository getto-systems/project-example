use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::{
    auth_ticket::_auth::validate::{
        event::ValidateAuthTokenEvent,
        infra::{ValidateApiTokenRequestDecoder, ValidateAuthTokenInfra},
        method::validate_api_token,
    },
    auth_user::_common::kernel::data::AuthUser,
};

pub enum ValidateApiTokenState {
    Validate(ValidateAuthTokenEvent),
    Success(AuthUser),
}

impl std::fmt::Display for ValidateApiTokenState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => write!(f, "{}", event),
            Self::Success(user) => write!(f, "validate api token success; {}", user),
        }
    }
}

pub trait ValidateApiTokenMaterial {
    type Validate: ValidateAuthTokenInfra;

    fn validate(&self) -> &Self::Validate;
}

pub struct ValidateApiTokenAction<M: ValidateApiTokenMaterial> {
    pubsub: ActionStatePubSub<ValidateApiTokenState>,
    material: M,
}

impl<M: ValidateApiTokenMaterial> ValidateApiTokenAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&ValidateApiTokenState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        request_decoder: impl ValidateApiTokenRequestDecoder,
    ) -> MethodResult<ValidateApiTokenState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let require_roles = request_decoder.decode();

        let ticket = validate_api_token(m.validate(), require_roles, |event| {
            pubsub.post(ValidateApiTokenState::Validate(event))
        })
        .await?;

        Ok(pubsub.post(ValidateApiTokenState::Success(ticket.into_user())))
    }
}
