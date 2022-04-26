use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{
    authenticate, AuthenticateEvent, AuthenticateInfra,
};

use crate::auth::user::password::reset::token_destination::change::infra::{
    ChangeResetTokenDestinationFields, ChangeResetTokenDestinationFieldsExtract,
    ChangeResetTokenDestinationRepository, ChangeResetTokenDestinationRequestDecoder,
};

use crate::{
    auth::{
        data::RequireAuthRoles, ticket::kernel::data::PermissionError,
        user::password::reset::token_destination::change::data::ValidateChangeResetTokenDestinationFieldsError,
    },
    z_lib::repository::data::RepositoryError,
};

pub enum ChangeResetTokenDestinationState {
    Authenticate(AuthenticateEvent),
    PermissionError(PermissionError),
    ChangeDestination(ChangeResetTokenDestinationEvent),
}

impl std::fmt::Display for ChangeResetTokenDestinationState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authenticate(event) => event.fmt(f),
            Self::PermissionError(err) => err.fmt(f),
            Self::ChangeDestination(event) => event.fmt(f),
        }
    }
}

pub trait ChangeResetTokenDestinationMaterial {
    type Authenticate: AuthenticateInfra;

    type DestinationRepository: ChangeResetTokenDestinationRepository;

    fn authenticate(&self) -> &Self::Authenticate;

    fn destination_repository(&self) -> &Self::DestinationRepository;
}

pub struct ChangeResetTokenDestinationAction<
    R: ChangeResetTokenDestinationRequestDecoder,
    M: ChangeResetTokenDestinationMaterial,
> {
    pubsub: ActionStatePubSub<ChangeResetTokenDestinationState>,
    request_decoder: R,
    material: M,
}

impl<R: ChangeResetTokenDestinationRequestDecoder, M: ChangeResetTokenDestinationMaterial>
    ChangeResetTokenDestinationAction<R, M>
{
    pub fn with_material(request_decoder: R, material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            request_decoder,
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&ChangeResetTokenDestinationState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<ChangeResetTokenDestinationState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = self.request_decoder.decode();

        let ticket = authenticate(m.authenticate(), |event| {
            pubsub.post(ChangeResetTokenDestinationState::Authenticate(event))
        })
        .await?;

        ticket
            .check_enough_permission(RequireAuthRoles::user())
            .map_err(|err| pubsub.post(ChangeResetTokenDestinationState::PermissionError(err)))?;

        change_destination(&m, fields, |event| {
            pubsub.post(ChangeResetTokenDestinationState::ChangeDestination(event))
        })
        .await
    }
}

pub enum ChangeResetTokenDestinationEvent {
    Success,
    Invalid(ValidateChangeResetTokenDestinationFieldsError),
    NotFound,
    Conflict,
    RepositoryError(RepositoryError),
}

mod change_reset_token_destination_event {
    use super::ChangeResetTokenDestinationEvent;

    const SUCCESS: &'static str = "change reset token destination success";
    const ERROR: &'static str = "change reset token destination error";

    impl std::fmt::Display for ChangeResetTokenDestinationEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Success => write!(f, "{}", SUCCESS),
                Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
                Self::NotFound => write!(f, "{}; not found", ERROR),
                Self::Conflict => write!(f, "{}; changes conflicted", ERROR),
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

async fn change_destination<S>(
    infra: &impl ChangeResetTokenDestinationMaterial,
    fields: ChangeResetTokenDestinationFieldsExtract,
    post: impl Fn(ChangeResetTokenDestinationEvent) -> S,
) -> MethodResult<S> {
    let fields = ChangeResetTokenDestinationFields::convert(fields)
        .map_err(|err| post(ChangeResetTokenDestinationEvent::Invalid(err)))?;

    let destination_repository = infra.destination_repository();

    let stored_destination = destination_repository
        .lookup_destination(&fields.login_id)
        .await
        .map_err(|err| post(ChangeResetTokenDestinationEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ChangeResetTokenDestinationEvent::NotFound))?;

    if stored_destination != fields.from {
        return Err(post(ChangeResetTokenDestinationEvent::Conflict));
    }

    destination_repository
        .change_destination(fields.login_id, fields.to)
        .await
        .map_err(|err| post(ChangeResetTokenDestinationEvent::RepositoryError(err)))?;

    Ok(post(ChangeResetTokenDestinationEvent::Success))
}
