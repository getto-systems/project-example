use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{
    validate_auth_token, ValidateAuthTokenEvent, ValidateAuthTokenInfra,
};

use crate::auth::user::password::reset::token_destination::change::infra::{
    ChangeResetTokenDestinationFields, ChangeResetTokenDestinationRepository,
    ChangeResetTokenDestinationRequestDecoder,
};

use crate::{
    auth::user::password::reset::{
        kernel::data::ResetTokenDestination,
        token_destination::change::data::ValidateChangeResetTokenDestinationFieldsError,
    },
    z_lib::repository::data::RepositoryError,
};

pub enum ChangeResetTokenDestinationState {
    Validate(ValidateAuthTokenEvent),
    ChangeDestination(ChangeResetTokenDestinationEvent),
}

impl std::fmt::Display for ChangeResetTokenDestinationState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => event.fmt(f),
            Self::ChangeDestination(event) => event.fmt(f),
        }
    }
}

pub trait ChangeResetTokenDestinationMaterial {
    type Validate: ValidateAuthTokenInfra;

    type DestinationRepository: ChangeResetTokenDestinationRepository;

    fn validate(&self) -> &Self::Validate;

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

        validate_auth_token(m.validate(), |event| {
            pubsub.post(ChangeResetTokenDestinationState::Validate(event))
        })
        .await?;

        change_destination(&m, fields, |event| {
            pubsub.post(ChangeResetTokenDestinationState::ChangeDestination(event))
        })
        .await
    }
}

pub enum ChangeResetTokenDestinationEvent {
    Success(ResetTokenDestination),
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
                Self::Success(destination) => write!(f, "{}; {}", SUCCESS, destination),
                Self::Invalid(err) => err.fmt(f),
                Self::NotFound => write!(f, "{}; not found", ERROR),
                Self::Conflict => write!(f, "{}; changes conflicted", ERROR),
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

async fn change_destination<S>(
    infra: &impl ChangeResetTokenDestinationMaterial,
    fields: Result<
        ChangeResetTokenDestinationFields,
        ValidateChangeResetTokenDestinationFieldsError,
    >,
    post: impl Fn(ChangeResetTokenDestinationEvent) -> S,
) -> MethodResult<S> {
    let fields = fields.map_err(|err| post(ChangeResetTokenDestinationEvent::Invalid(err)))?;

    let destination_repository = infra.destination_repository();

    let (_user_id, stored_destination) = destination_repository
        .lookup_destination(&fields.login_id)
        .await
        .map_err(|err| post(ChangeResetTokenDestinationEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ChangeResetTokenDestinationEvent::NotFound))?;

    if stored_destination != fields.from {
        return Err(post(ChangeResetTokenDestinationEvent::Conflict));
    }

    destination_repository
        .change_destination(&fields.login_id, fields.to)
        .await
        .map_err(|err| post(ChangeResetTokenDestinationEvent::RepositoryError(err)))?;

    let updated_destination = destination_repository
        .get_updated_destination(&fields.login_id)
        .await
        .map_err(|err| post(ChangeResetTokenDestinationEvent::RepositoryError(err)))?;

    Ok(post(ChangeResetTokenDestinationEvent::Success(
        updated_destination,
    )))
}
