use std::fmt::Display;

use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::{
    auth_ticket::_api::{
        encode::{
            event::EncodeAuthTicketEvent, infra::EncodeAuthTicketInfra, method::encode_auth_ticket,
        },
        issue::{
            event::IssueAuthTicketEvent, infra::IssueAuthTicketInfra, method::issue_auth_ticket,
        },
    },
    password::reset::_api::reset::{
        event::ResetPasswordEvent, infra::ResetPasswordInfra, method::reset_password,
    },
};

pub enum ResetPasswordState {
    Reset(ResetPasswordEvent),
    Issue(IssueAuthTicketEvent),
    Encode(EncodeAuthTicketEvent),
}

impl Display for ResetPasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reset(event) => write!(f, "{}", event),
            Self::Issue(event) => write!(f, "{}", event),
            Self::Encode(event) => write!(f, "{}", event),
        }
    }
}

pub trait ResetPasswordMaterial {
    type Reset: ResetPasswordInfra;
    type Issue: IssueAuthTicketInfra;
    type Encode: EncodeAuthTicketInfra;

    fn reset(&self) -> &Self::Reset;
    fn issue(&self) -> &Self::Issue;
    fn encode(&self) -> &Self::Encode;
}

pub struct ResetPasswordAction<M: ResetPasswordMaterial> {
    pubsub: ActionStatePubSub<ResetPasswordState>,
    material: M,
}

impl<M: ResetPasswordMaterial> ResetPasswordAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&ResetPasswordState)) {
        self.pubsub.subscribe(handler);
    }

    pub fn ignite(self) -> MethodResult<ResetPasswordState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let user = reset_password(m.reset(), |event| {
            pubsub.post(ResetPasswordState::Reset(event))
        })?;

        let ticket = issue_auth_ticket(m.issue(), user, |event| {
            pubsub.post(ResetPasswordState::Issue(event))
        })?;

        encode_auth_ticket(m.encode(), ticket, |event| {
            pubsub.post(ResetPasswordState::Encode(event))
        })
    }
}
