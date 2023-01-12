use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::common::proxy::action::CoreProxyParams;

use crate::auth::ticket::authorize::proxy::{authorize, AuthorizeEvent, AuthorizeInfra};

use crate::auth::user::password::reset::token_destination::change::infra::{
    ChangeResetTokenDestinationFieldsExtract, ChangeResetTokenDestinationRepository,
};

use crate::{
    auth::{
        ticket::kernel::data::{AuthPermissionRequired, AuthorizeTokenExtract},
        user::password::reset::token_destination::change::data::ValidateChangeResetTokenDestinationFieldsError,
    },
    common::api::repository::data::RepositoryError,
};

pub enum ChangeResetTokenDestinationState {
    Authorize(AuthorizeEvent),
    ChangeDestination(ChangeResetTokenDestinationEvent),
}

impl std::fmt::Display for ChangeResetTokenDestinationState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorize(event) => event.fmt(f),
            Self::ChangeDestination(event) => event.fmt(f),
        }
    }
}

pub trait ChangeResetTokenDestinationMaterial {
    type Authorize: AuthorizeInfra;

    type DestinationRepository: ChangeResetTokenDestinationRepository;

    fn authorize(&self) -> &Self::Authorize;

    fn destination_repository(&self) -> &Self::DestinationRepository;
}

pub struct ChangeResetTokenDestinationAction<M: ChangeResetTokenDestinationMaterial> {
    pub info: ChangeResetTokenDestinationActionInfo,
    pubsub: ActionStatePubSub<ChangeResetTokenDestinationState>,
    material: M,
}

pub struct ChangeResetTokenDestinationActionInfo;

impl ChangeResetTokenDestinationActionInfo {
    pub const fn name(&self) -> &'static str {
        "auth.user.password.reset.token-destination.change"
    }

    pub fn required(&self) -> AuthPermissionRequired {
        AuthPermissionRequired::user()
    }

    pub fn params(&self) -> CoreProxyParams {
        (self.name(), self.required())
    }
}

impl<M: ChangeResetTokenDestinationMaterial> ChangeResetTokenDestinationAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: ChangeResetTokenDestinationActionInfo,
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&ChangeResetTokenDestinationState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        token: impl AuthorizeTokenExtract,
        fields: impl ChangeResetTokenDestinationFieldsExtract,
    ) -> MethodResult<ChangeResetTokenDestinationState> {
        authorize(
            self.material.authorize(),
            (token, self.info.required()),
            |event| {
                self.pubsub
                    .post(ChangeResetTokenDestinationState::Authorize(event))
            },
        )
        .await?;

        change_destination(&self.material, fields, |event| {
            self.pubsub
                .post(ChangeResetTokenDestinationState::ChangeDestination(event))
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
    fields: impl ChangeResetTokenDestinationFieldsExtract,
    post: impl Fn(ChangeResetTokenDestinationEvent) -> S,
) -> MethodResult<S> {
    let fields = fields
        .convert()
        .map_err(|err| post(ChangeResetTokenDestinationEvent::Invalid(err)))?;

    let stored_destination = infra
        .destination_repository()
        .lookup_destination(&fields.login_id)
        .await
        .map_err(|err| post(ChangeResetTokenDestinationEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ChangeResetTokenDestinationEvent::NotFound))?;

    if stored_destination != fields.from {
        return Err(post(ChangeResetTokenDestinationEvent::Conflict));
    }

    infra
        .destination_repository()
        .change_destination(fields.login_id, fields.to)
        .await
        .map_err(|err| post(ChangeResetTokenDestinationEvent::RepositoryError(err)))?;

    Ok(post(ChangeResetTokenDestinationEvent::Success))
}
