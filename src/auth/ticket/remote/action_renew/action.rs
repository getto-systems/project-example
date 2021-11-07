use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::remote::{
    encode::{
        event::EncodeAuthTicketEvent, infra::EncodeAuthTicketInfra, method::encode_auth_ticket,
    },
    validate::{
        event::ValidateAuthTokenEvent, infra::ValidateAuthTokenInfra, method::validate_auth_token,
    },
};

use crate::auth::user::remote::kernel::data::RequireAuthRoles;

pub enum RenewAuthTicketState {
    Validate(ValidateAuthTokenEvent),
    Encode(EncodeAuthTicketEvent),
}

impl std::fmt::Display for RenewAuthTicketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => write!(f, "{}", event),
            Self::Encode(event) => write!(f, "{}", event),
        }
    }
}

pub trait RenewAuthTicketMaterial {
    type Validate: ValidateAuthTokenInfra;
    type Encode: EncodeAuthTicketInfra;

    fn validate(&self) -> &Self::Validate;
    fn encode(&self) -> &Self::Encode;
}

pub struct RenewAuthTicketAction<M: RenewAuthTicketMaterial> {
    pubsub: ActionStatePubSub<RenewAuthTicketState>,
    material: M,
}

impl<M: RenewAuthTicketMaterial> RenewAuthTicketAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&RenewAuthTicketState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<RenewAuthTicketState> {
        let pubsub = self.pubsub;
        let m = self.material;

        // encode_auth_ticket は環境から ticket を取り出すのではなく、 ticket を encode するのだ
        let ticket = validate_auth_token(m.validate(), RequireAuthRoles::Nothing, |event| {
            pubsub.post(RenewAuthTicketState::Validate(event))
        })
        .await?;

        encode_auth_ticket(m.encode(), ticket, |event| {
            pubsub.post(RenewAuthTicketState::Encode(event))
        })
        .await
    }
}
