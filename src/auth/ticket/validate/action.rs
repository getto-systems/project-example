use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{authenticate, AuthenticateEvent, AuthenticateInfra};

use crate::auth::ticket::validate::infra::AuthorizeRequestDecoder;

use crate::auth::{ticket::kernel::data::ValidateAuthRolesError, user::kernel::data::AuthUser};

pub enum AuthorizeState {
    Authenticate(AuthenticateEvent),
    PermissionError(ValidateAuthRolesError),
    Success(AuthUser),
}

impl std::fmt::Display for AuthorizeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authenticate(event) => event.fmt(f),
            Self::PermissionError(err) => err.fmt(f),
            Self::Success(user) => write!(f, "authorize success; {}", user),
        }
    }
}

pub struct AuthorizeAction<R: AuthorizeRequestDecoder, M: AuthenticateInfra> {
    pubsub: ActionStatePubSub<AuthorizeState>,
    request_decoder: R,
    material: M,
}

impl<R: AuthorizeRequestDecoder, M: AuthenticateInfra> AuthorizeAction<R, M> {
    pub fn with_material(request_decoder: R, material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            request_decoder,
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&AuthorizeState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<AuthorizeState> {
        let pubsub = self.pubsub;

        let require_roles = self.request_decoder.decode();

        let ticket = authenticate(&self.material, |event| {
            pubsub.post(AuthorizeState::Authenticate(event))
        })
        .await?;

        let ticket = ticket
            .check_enough_permission(require_roles)
            .map_err(|err| pubsub.post(AuthorizeState::PermissionError(err)))?;

        Ok(pubsub.post(AuthorizeState::Success(ticket.into_user())))
    }
}
