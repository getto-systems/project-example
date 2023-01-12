use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::authenticate::method::{
    authenticate_with_token, AuthenticateWithTokenEvent, AuthenticateWithTokenInfra,
};

use crate::auth::ticket::logout::infra::LogoutAuthTicketRepository;

use crate::{
    auth::ticket::kernel::data::{AuthTicket, AuthenticateTokenExtract},
    common::api::repository::data::RepositoryError,
};

pub enum LogoutState {
    AuthenticateWithToken(AuthenticateWithTokenEvent),
    Logout(LogoutEvent),
}

impl std::fmt::Display for LogoutState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthenticateWithToken(event) => event.fmt(f),
            Self::Logout(event) => event.fmt(f),
        }
    }
}

pub trait LogoutMaterial {
    type AuthenticateWithToken: AuthenticateWithTokenInfra;
    type TicketRepository: LogoutAuthTicketRepository;

    fn authenticate_with_token(&self) -> &Self::AuthenticateWithToken;
    fn ticket_repository(&self) -> &Self::TicketRepository;
}

pub struct LogoutAction<M: LogoutMaterial> {
    pub info: LogoutActionInfo,
    pubsub: ActionStatePubSub<LogoutState>,
    material: M,
}

pub struct LogoutActionInfo;

impl LogoutActionInfo {
    pub const fn name(&self) -> &'static str {
        "auth.ticket.logout"
    }
}

impl<M: LogoutMaterial> LogoutAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: LogoutActionInfo,
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&LogoutState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self, token: impl AuthenticateTokenExtract) -> MethodResult<LogoutState> {
        let (ticket, _token) =
            authenticate_with_token(self.material.authenticate_with_token(), token, |event| {
                self.pubsub.post(LogoutState::AuthenticateWithToken(event))
            })
            .await?;

        logout(&self.material, ticket, |event| {
            self.pubsub.post(LogoutState::Logout(event))
        })
        .await
    }
}

pub enum LogoutEvent {
    Success,
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "logout success";
const ERROR: &'static str = "logout error";

impl std::fmt::Display for LogoutEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "{}", SUCCESS),
            Self::RepositoryError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}

async fn logout<S>(
    infra: &impl LogoutMaterial,
    ticket: AuthTicket,
    post: impl Fn(LogoutEvent) -> S,
) -> MethodResult<S> {
    infra
        .ticket_repository()
        .discard(&ticket)
        .await
        .map_err(|err| post(LogoutEvent::RepositoryError(err)))?;

    Ok(post(LogoutEvent::Success))
}
