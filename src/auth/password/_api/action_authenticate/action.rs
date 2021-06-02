use std::fmt::Display;

use getto_application::{data::MethodResult, infra::PubSub};

use crate::auth::{
    auth_ticket::_api::{
        encode::{
            event::EncodeAuthTicketEvent, infra::EncodeAuthTicketInfra, method::encode_auth_ticket,
        },
        issue::{
            event::IssueAuthTicketEvent, infra::IssueAuthTicketInfra, method::issue_auth_ticket,
        },
    },
    password::_api::authenticate::{
        event::AuthenticatePasswordEvent, infra::AuthenticatePasswordInfra,
        method::authenticate_password,
    },
};

pub enum AuthenticatePasswordState {
    Authenticate(AuthenticatePasswordEvent),
    Issue(IssueAuthTicketEvent),
    Encode(EncodeAuthTicketEvent),
}

impl Display for AuthenticatePasswordState {
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
    pubsub: PubSub<AuthenticatePasswordState>,
    material: M,
}

impl<M: AuthenticatePasswordMaterial> AuthenticatePasswordAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: PubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&AuthenticatePasswordState)) {
        self.pubsub.subscribe(handler);
    }

    pub fn ignite(self) -> MethodResult<AuthenticatePasswordState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let user = authenticate_password(m.authenticate(), |event| {
            pubsub.post(AuthenticatePasswordState::Authenticate(event))
        })?;

        let ticket = issue_auth_ticket(m.issue(), user, |event| {
            pubsub.post(AuthenticatePasswordState::Issue(event))
        })?;

        encode_auth_ticket(m.encode(), ticket, |event| {
            pubsub.post(AuthenticatePasswordState::Encode(event))
        })
    }
}
