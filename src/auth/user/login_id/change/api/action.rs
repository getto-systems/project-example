use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{
    authenticate, AuthenticateEvent, AuthenticateInfra,
};

use crate::auth::user::login_id::change::infra::{
    OverrideLoginIdFields, OverrideLoginIdFieldsExtract, OverrideLoginIdRepository,
    OverrideLoginIdRequestDecoder,
};

use crate::{
    auth::user::login_id::change::data::ValidateOverrideLoginIdFieldsError,
    z_lib::repository::data::RepositoryError,
};

pub enum OverrideLoginIdState {
    Authenticate(AuthenticateEvent),
    Override(OverrideLoginIdEvent),
}

impl std::fmt::Display for OverrideLoginIdState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authenticate(event) => event.fmt(f),
            Self::Override(event) => event.fmt(f),
        }
    }
}

pub trait OverrideLoginIdMaterial {
    type Authenticate: AuthenticateInfra;

    type LoginIdRepository: OverrideLoginIdRepository;

    fn authenticate(&self) -> &Self::Authenticate;

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

        authenticate(m.authenticate(), |event| {
            pubsub.post(OverrideLoginIdState::Authenticate(event))
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
    Invalid(ValidateOverrideLoginIdFieldsError),
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
    let fields = OverrideLoginIdFields::convert(fields)
        .map_err(|err| post(OverrideLoginIdEvent::Invalid(err)))?;

    let login_id_repository = infra.login_id_repository();

    if login_id_repository
        .check_login_id_registered(&fields.new_login_id)
        .await
        .map_err(|err| post(OverrideLoginIdEvent::RepositoryError(err)))?
    {
        return Err(post(OverrideLoginIdEvent::AlreadyRegistered));
    }

    let user = login_id_repository
        .lookup_user(&fields.login_id)
        .await
        .map_err(|err| post(OverrideLoginIdEvent::RepositoryError(err)))?
        .ok_or_else(|| post(OverrideLoginIdEvent::NotFound))?;

    login_id_repository
        .override_login_id(fields.new_login_id, user)
        .await
        .map_err(|err| post(OverrideLoginIdEvent::RepositoryError(err)))?;

    Ok(post(OverrideLoginIdEvent::Success))
}
