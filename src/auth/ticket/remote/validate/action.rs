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

pub trait ValidateApiTokenMaterial {
    type RequestDecoder: ValidateApiTokenRequestDecoder;
    type Infra: ValidateAuthTokenInfra;

    fn extract(self) -> (Self::RequestDecoder, Self::Infra);
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

    pub async fn ignite(self) -> MethodResult<ValidateApiTokenState> {
        let pubsub = self.pubsub;
        let (request_decoder, infra) = self.material.extract();

        let require_roles = request_decoder.decode();

        let ticket = validate_auth_token(&infra, require_roles, |event| {
            pubsub.post(ValidateApiTokenState::Validate(event))
        })
        .await?;

        Ok(pubsub.post(ValidateApiTokenState::Success(ticket.into_user())))
    }
}
