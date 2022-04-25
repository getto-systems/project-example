use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{authenticate, AuthenticateEvent, AuthenticateInfra};

use crate::auth::ticket::validate::infra::AuthenticateApiRequestDecoder;

use crate::auth::{ticket::kernel::data::PermissionError, user::kernel::data::AuthUser};

pub enum AuthenticateApiState {
    Authenticate(AuthenticateEvent),
    PermissionError(PermissionError),
    Success(AuthUser),
}

impl std::fmt::Display for AuthenticateApiState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authenticate(event) => event.fmt(f),
            Self::PermissionError(err) => err.fmt(f),
            Self::Success(user) => write!(f, "authorize success; {}", user),
        }
    }
}

pub struct AuthenticateApiAction<R: AuthenticateApiRequestDecoder, M: AuthenticateInfra> {
    pubsub: ActionStatePubSub<AuthenticateApiState>,
    request_decoder: R,
    material: M,
}

impl<R: AuthenticateApiRequestDecoder, M: AuthenticateInfra> AuthenticateApiAction<R, M> {
    pub fn with_material(request_decoder: R, material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            request_decoder,
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&AuthenticateApiState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<AuthenticateApiState> {
        let pubsub = self.pubsub;

        let require_roles = self.request_decoder.decode();

        let ticket = authenticate(&self.material, |event| {
            pubsub.post(AuthenticateApiState::Authenticate(event))
        })
        .await?;

        let ticket = ticket
            .check_enough_permission(require_roles)
            .map_err(|err| pubsub.post(AuthenticateApiState::PermissionError(err)))?;

        Ok(pubsub.post(AuthenticateApiState::Success(ticket.into_user())))
    }
}
