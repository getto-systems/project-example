use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{
    validate_auth_token, ValidateAuthTokenEvent, ValidateAuthTokenInfra,
};

use crate::auth::user::login_id::change::infra::{
    OverrideLoginIdFieldsExtract, OverrideLoginIdRepository, OverrideLoginIdRequestDecoder,
};

use crate::{
    auth::user::login_id::kernel::data::{LoginId, ValidateLoginIdError},
    z_lib::repository::data::RepositoryError,
};

pub enum OverrideLoginIdState {
    Validate(ValidateAuthTokenEvent),
    Override(OverrideLoginIdEvent),
}

impl std::fmt::Display for OverrideLoginIdState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => event.fmt(f),
            Self::Override(event) => event.fmt(f),
        }
    }
}

pub trait OverrideLoginIdMaterial {
    type Validate: ValidateAuthTokenInfra;

    type LoginIdRepository: OverrideLoginIdRepository;

    fn validate(&self) -> &Self::Validate;

    fn login_id_repository(&self) -> &Self::LoginIdRepository;
}

pub struct OverrideLoginIdAction<R: OverrideLoginIdRequestDecoder, M: OverrideLoginIdMaterial> {
    pubsub: ActionStatePubSub<OverrideLoginIdState>,
    request_decoder: R,
    material: M,
}

impl<R: OverrideLoginIdRequestDecoder, M: OverrideLoginIdMaterial> OverrideLoginIdAction<R, M> {
    pub fn with_material(request_decoder: R, material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            request_decoder,
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&OverrideLoginIdState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<OverrideLoginIdState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = self.request_decoder.decode();

        validate_auth_token(m.validate(), |event| {
            pubsub.post(OverrideLoginIdState::Validate(event))
        })
        .await?;

        override_login_id(&m, fields, |event| {
            pubsub.post(OverrideLoginIdState::Override(event))
        })
        .await
    }
}

pub enum OverrideLoginIdEvent {
    Success,
    Invalid(ValidateLoginIdError),
    NotFound,
    AlreadyRegistered,
    RepositoryError(RepositoryError),
}

mod override_login_id_event {
    use super::OverrideLoginIdEvent;

    const SUCCESS: &'static str = "override login-id success";
    const ERROR: &'static str = "override login-id error";

    impl std::fmt::Display for OverrideLoginIdEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Success => write!(f, "{}", SUCCESS),
                Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
                Self::NotFound => write!(f, "{}; not found", ERROR),
                Self::AlreadyRegistered => {
                    write!(f, "{}; new login id is already registered", ERROR)
                }
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

async fn override_login_id<S>(
    infra: &impl OverrideLoginIdMaterial,
    fields: OverrideLoginIdFieldsExtract,
    post: impl Fn(OverrideLoginIdEvent) -> S,
) -> MethodResult<S> {
    let login_id = LoginId::validate(fields.login_id)
        .map_err(|err| post(OverrideLoginIdEvent::Invalid(err)))?;
    let new_login_id = LoginId::validate(fields.new_login_id)
        .map_err(|err| post(OverrideLoginIdEvent::Invalid(err)))?;

    let login_id_repository = infra.login_id_repository();

    if login_id_repository
        .check_login_id_registered(&new_login_id)
        .await
        .map_err(|err| post(OverrideLoginIdEvent::RepositoryError(err)))?
    {
        return Err(post(OverrideLoginIdEvent::AlreadyRegistered));
    }

    let user = login_id_repository
        .lookup_user(&login_id)
        .await
        .map_err(|err| post(OverrideLoginIdEvent::RepositoryError(err)))?
        .ok_or_else(|| post(OverrideLoginIdEvent::NotFound))?;

    login_id_repository
        .override_login_id(user, new_login_id)
        .await
        .map_err(|err| post(OverrideLoginIdEvent::RepositoryError(err)))?;

    Ok(post(OverrideLoginIdEvent::Success))
}
