use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::{
    auth_ticket::_auth::{
        encode::{
            event::EncodeAuthTicketEvent, infra::EncodeAuthTicketInfra, method::encode_auth_ticket,
        },
        issue::{
            event::IssueAuthTicketEvent, infra::IssueAuthTicketInfra, method::issue_auth_ticket,
        },
    },
    password::_auth::authenticate::{
        event::AuthenticatePasswordEvent,
        infra::{AuthenticatePasswordInfra, AuthenticatePasswordRequestDecoder},
        method::authenticate_password,
    },
};

pub enum AuthenticatePasswordState {
    Authenticate(AuthenticatePasswordEvent),
    Issue(IssueAuthTicketEvent),
    Encode(EncodeAuthTicketEvent),
}

impl std::fmt::Display for AuthenticatePasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authenticate(event) => write!(f, "{}", event),
            Self::Issue(event) => write!(f, "{}", event),
            Self::Encode(event) => write!(f, "{}", event),
        }
    }
}

pub trait AuthenticatePasswordMaterial {
    type Authenticate: AuthenticatePasswordInfra;
    type Issue: IssueAuthTicketInfra;
    type Encode: EncodeAuthTicketInfra;

    fn authenticate(&self) -> &Self::Authenticate;
    fn issue(&self) -> &Self::Issue;
    fn encode(&self) -> &Self::Encode;
}

pub struct AuthenticatePasswordAction<M: AuthenticatePasswordMaterial> {
    pubsub: ActionStatePubSub<AuthenticatePasswordState>,
    material: M,
}

impl<M: AuthenticatePasswordMaterial> AuthenticatePasswordAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthenticatePasswordState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        request: impl AuthenticatePasswordRequestDecoder,
    ) -> MethodResult<AuthenticatePasswordState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = request.decode();

        let user = authenticate_password(m.authenticate(), fields, |event| {
            pubsub.post(AuthenticatePasswordState::Authenticate(event))
        })
        .await?;

        let ticket = issue_auth_ticket(m.issue(), user, |event| {
            pubsub.post(AuthenticatePasswordState::Issue(event))
        })
        .await?;

        encode_auth_ticket(m.encode(), ticket, |event| {
            pubsub.post(AuthenticatePasswordState::Encode(event))
        })
        .await
    }
}
