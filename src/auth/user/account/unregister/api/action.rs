use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{
    validate_auth_token, ValidateAuthTokenEvent, ValidateAuthTokenInfra,
};

use crate::auth::user::account::unregister::infra::{
    DiscardAuthTicketRepository, UnregisterAuthUserAccountRepository,
    UnregisterAuthUserAccountRequestDecoder,
};

use crate::{
    auth::{
        data::RequireAuthRoles,
        ticket::kernel::data::ValidateAuthRolesError,
        user::{
            account::unregister::data::ValidateUnregisterAuthUserAccountFieldsError,
            login_id::kernel::data::LoginId,
        },
    },
    z_lib::repository::data::RepositoryError,
};

pub enum UnregisterAuthUserAccountState {
    Validate(ValidateAuthTokenEvent),
    PermissionError(ValidateAuthRolesError),
    UnregisterUser(UnregisterAuthUserAccountEvent),
}

impl std::fmt::Display for UnregisterAuthUserAccountState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => event.fmt(f),
            Self::PermissionError(event) => event.fmt(f),
            Self::UnregisterUser(event) => event.fmt(f),
        }
    }
}

pub trait UnregisterAuthUserAccountMaterial {
    type Validate: ValidateAuthTokenInfra;

    type TicketRepository: DiscardAuthTicketRepository;
    type UserRepository: UnregisterAuthUserAccountRepository;

    fn validate(&self) -> &Self::Validate;

    fn ticket_repository(&self) -> &Self::TicketRepository;
    fn user_repository(&self) -> &Self::UserRepository;
}

pub struct UnregisterAuthUserAccountAction<
    R: UnregisterAuthUserAccountRequestDecoder,
    M: UnregisterAuthUserAccountMaterial,
> {
    pubsub: ActionStatePubSub<UnregisterAuthUserAccountState>,
    request_decoder: R,
    material: M,
}

impl<R: UnregisterAuthUserAccountRequestDecoder, M: UnregisterAuthUserAccountMaterial>
    UnregisterAuthUserAccountAction<R, M>
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
        handler: impl 'static + Fn(&UnregisterAuthUserAccountState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<UnregisterAuthUserAccountState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = self.request_decoder.decode();

        let ticket = validate_auth_token(m.validate(), |event| {
            pubsub.post(UnregisterAuthUserAccountState::Validate(event))
        })
        .await?;

        ticket
            .check_enough_permission(RequireAuthRoles::user())
            .map_err(|err| pubsub.post(UnregisterAuthUserAccountState::PermissionError(err)))?;

        unregister_user(&m, fields, |event| {
            pubsub.post(UnregisterAuthUserAccountState::UnregisterUser(event))
        })
        .await
    }
}

pub enum UnregisterAuthUserAccountEvent {
    Success,
    Invalid(ValidateUnregisterAuthUserAccountFieldsError),
    RepositoryError(RepositoryError),
}

mod unregister_auth_user_account_event {
    use super::UnregisterAuthUserAccountEvent;

    const SUCCESS: &'static str = "unregister auth user account success";
    const ERROR: &'static str = "unregister auth user account error";

    impl std::fmt::Display for UnregisterAuthUserAccountEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Success => write!(f, "{}", SUCCESS),
                Self::Invalid(err) => err.fmt(f),
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

async fn unregister_user<S>(
    infra: &impl UnregisterAuthUserAccountMaterial,
    fields: Result<LoginId, ValidateUnregisterAuthUserAccountFieldsError>,
    post: impl Fn(UnregisterAuthUserAccountEvent) -> S,
) -> MethodResult<S> {
    let login_id = fields.map_err(|err| post(UnregisterAuthUserAccountEvent::Invalid(err)))?;

    let ticket_repository = infra.ticket_repository();
    let user_repository = infra.user_repository();

    if let Some(user_id) = user_repository
        .lookup_user_id(&login_id)
        .await
        .map_err(|err| post(UnregisterAuthUserAccountEvent::RepositoryError(err)))?
    {
        user_repository
            .unregister_user(&user_id, &login_id)
            .await
            .map_err(|err| post(UnregisterAuthUserAccountEvent::RepositoryError(err)))?;

        ticket_repository
            .discard_all(&user_id)
            .await
            .map_err(|err| post(UnregisterAuthUserAccountEvent::RepositoryError(err)))?;
    }

    Ok(post(UnregisterAuthUserAccountEvent::Success))
}
