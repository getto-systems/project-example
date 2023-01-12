use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::authorize::method::{
    authorize_with_token, AuthorizeWithTokenEvent, AuthorizeWithTokenInfra,
};

use crate::auth::{
    kernel::infra::AuthClock,
    ticket::authorize::infra::{
        AuthorizeFieldsExtract, ClarifyAuthorizeTokenAuthTicketRepository,
        ClarifyAuthorizeTokenAuthUserRepository,
    },
};

use crate::{
    auth::ticket::kernel::data::{AuthPermissionError, AuthPermissionRequired, AuthTicket},
    common::api::repository::data::RepositoryError,
};

pub enum ClarifyAuthorizeTokenState {
    AuthorizeWithToken(AuthorizeWithTokenEvent),
    ClarifyAuthorizeToken(ClarifyAuthorizeTokenEvent),
}

impl std::fmt::Display for ClarifyAuthorizeTokenState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthorizeWithToken(event) => event.fmt(f),
            Self::ClarifyAuthorizeToken(event) => event.fmt(f),
        }
    }
}

pub trait ClarifyAuthorizeTokenMaterial {
    type AuthorizeWithToken: AuthorizeWithTokenInfra;
    type Clock: AuthClock;
    type TicketRepository: ClarifyAuthorizeTokenAuthTicketRepository;
    type UserRepository: ClarifyAuthorizeTokenAuthUserRepository;

    fn authorize_with_token(&self) -> &Self::AuthorizeWithToken;
    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
    fn user_repository(&self) -> &Self::UserRepository;
}

pub struct ClarifyAuthorizeTokenAction<M: ClarifyAuthorizeTokenMaterial> {
    pub info: ClarifyAuthorizeTokenActionInfo,
    pubsub: ActionStatePubSub<ClarifyAuthorizeTokenState>,
    material: M,
}

pub struct ClarifyAuthorizeTokenActionInfo;

impl ClarifyAuthorizeTokenActionInfo {
    pub const fn name(&self) -> &'static str {
        "auth.ticket.authorize.clarify"
    }
}

impl<M: ClarifyAuthorizeTokenMaterial> ClarifyAuthorizeTokenAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: ClarifyAuthorizeTokenActionInfo,
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&ClarifyAuthorizeTokenState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        fields: impl AuthorizeFieldsExtract,
    ) -> MethodResult<ClarifyAuthorizeTokenState> {
        let pubsub = self.pubsub;

        let (ticket, _token, required) =
            authorize_with_token(self.material.authorize_with_token(), fields, |event| {
                pubsub.post(ClarifyAuthorizeTokenState::AuthorizeWithToken(event))
            })
            .await?;

        clarify_authorize_token(&self.material, ticket, required, |event| {
            pubsub.post(ClarifyAuthorizeTokenState::ClarifyAuthorizeToken(event))
        })
        .await
    }
}

pub enum ClarifyAuthorizeTokenEvent {
    TicketNotFound,
    TicketHasExpired,
    UserNotFound,
    Success(AuthTicket),
    RepositoryError(RepositoryError),
    PermissionError(AuthPermissionError),
}

const SUCCESS: &'static str = "clarify authorize-token success";
const ERROR: &'static str = "clarify authorize-token error";

impl std::fmt::Display for ClarifyAuthorizeTokenEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TicketNotFound => write!(f, "{}; ticket not found", ERROR),
            Self::TicketHasExpired => write!(f, "{}; ticket has expired", ERROR),
            Self::UserNotFound => write!(f, "{}; user not found", ERROR),
            Self::Success(ticket) => {
                write!(f, "{}; {}", SUCCESS, ticket.attrs)
            }
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::PermissionError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

async fn clarify_authorize_token<S>(
    infra: &impl ClarifyAuthorizeTokenMaterial,
    ticket: AuthTicket,
    required: AuthPermissionRequired,
    post: impl Fn(ClarifyAuthorizeTokenEvent) -> S,
) -> MethodResult<S> {
    let expansion_limit = infra
        .ticket_repository()
        .lookup_expansion_limit(&ticket)
        .await
        .map_err(|err| post(ClarifyAuthorizeTokenEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ClarifyAuthorizeTokenEvent::TicketNotFound))?;

    if expansion_limit.has_elapsed(&infra.clock().now()) {
        return Err(post(ClarifyAuthorizeTokenEvent::TicketHasExpired));
    }

    let granted = infra
        .user_repository()
        .lookup_permission_granted(&ticket.attrs.user_id)
        .await
        .map_err(|err| post(ClarifyAuthorizeTokenEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ClarifyAuthorizeTokenEvent::UserNotFound))?;

    granted
        .has_enough_permission(&required)
        .map_err(|err| post(ClarifyAuthorizeTokenEvent::PermissionError(err)))?;

    Ok(post(ClarifyAuthorizeTokenEvent::Success(ticket)))
}
