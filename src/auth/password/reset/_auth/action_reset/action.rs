use std::fmt::Display;

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
    password::reset::_auth::reset::{
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

    fn extract(self) -> (Self::Reset, Self::Issue, Self::Encode);
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

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&ResetPasswordState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<ResetPasswordState> {
        let pubsub = self.pubsub;
        let (reset, issue, encode) = self.material.extract();

        let user =
            reset_password(reset, |event| pubsub.post(ResetPasswordState::Reset(event))).await?;

        let ticket = issue_auth_ticket(issue, user, |event| {
            pubsub.post(ResetPasswordState::Issue(event))
        })
        .await?;

        encode_auth_ticket(encode, ticket, |event| {
            pubsub.post(ResetPasswordState::Encode(event))
        })
        .await
    }
}
