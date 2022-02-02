use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::remote::validate::method::{
    validate_auth_token, ValidateAuthTokenEvent, ValidateAuthTokenInfra,
};

use crate::auth::ticket::{
    logout::remote::infra::LogoutAuthTicketRepository, remote::kernel::infra::AuthClock,
};

use crate::{
    auth::{
        ticket::remote::kernel::data::AuthTicket, user::remote::kernel::data::RequireAuthRoles,
    },
    z_lib::remote::repository::data::RepositoryError,
};

pub enum LogoutState {
    Validate(ValidateAuthTokenEvent),
    Logout(LogoutEvent),
}

impl std::fmt::Display for LogoutState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => event.fmt(f),
            Self::Logout(event) => event.fmt(f),
        }
    }
}

pub trait LogoutMaterial {
    type ValidateInfra: ValidateAuthTokenInfra;
    type Clock: AuthClock;
    type TicketRepository: LogoutAuthTicketRepository;

    fn validate(&self) -> &Self::ValidateInfra;
    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
}

pub struct LogoutAction<M: LogoutMaterial> {
    pubsub: ActionStatePubSub<LogoutState>,
    material: M,
}

impl<M: LogoutMaterial> LogoutAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&LogoutState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<LogoutState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let ticket = validate_auth_token(m.validate(), RequireAuthRoles::Nothing, |event| {
            pubsub.post(LogoutState::Validate(event))
        })
        .await?;

        logout(&m, ticket, |event| pubsub.post(LogoutState::Logout(event))).await
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
    let clock = infra.clock();
    let ticket_repository = infra.ticket_repository();

    ticket_repository
        .discard(ticket, clock.now())
        .await
        .map_err(|err| post(LogoutEvent::RepositoryError(err)))?;

    Ok(post(LogoutEvent::Success))
}
