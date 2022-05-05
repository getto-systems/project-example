use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{authenticate, AuthenticateEvent, AuthenticateInfra};

use crate::auth::user::login_id::change::infra::{
    OverwriteLoginIdFields, OverwriteLoginIdFieldsExtract, OverwriteLoginIdRepository,
    OverwriteLoginIdRequestDecoder,
};

use crate::{
    auth::user::login_id::change::data::ValidateOverwriteLoginIdFieldsError,
    z_lib::repository::data::RepositoryError,
};

pub enum OverwriteLoginIdState {
    Authenticate(AuthenticateEvent),
    Overwrite(OverwriteLoginIdEvent),
}

impl std::fmt::Display for OverwriteLoginIdState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authenticate(event) => event.fmt(f),
            Self::Overwrite(event) => event.fmt(f),
        }
    }
}

pub trait OverwriteLoginIdMaterial {
    type Authenticate: AuthenticateInfra;

    type LoginIdRepository: OverwriteLoginIdRepository;

    fn authenticate(&self) -> &Self::Authenticate;

    fn login_id_repository(&self) -> &Self::LoginIdRepository;
}

pub struct OverwriteLoginIdAction<R: OverwriteLoginIdRequestDecoder, M: OverwriteLoginIdMaterial> {
    pubsub: ActionStatePubSub<OverwriteLoginIdState>,
    request_decoder: R,
    material: M,
}

impl<R: OverwriteLoginIdRequestDecoder, M: OverwriteLoginIdMaterial> OverwriteLoginIdAction<R, M> {
    pub fn with_material(request_decoder: R, material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            request_decoder,
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&OverwriteLoginIdState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<OverwriteLoginIdState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = self.request_decoder.decode();

        authenticate(m.authenticate(), |event| {
            pubsub.post(OverwriteLoginIdState::Authenticate(event))
        })
        .await?;

        overwrite_login_id(&m, fields, |event| {
            pubsub.post(OverwriteLoginIdState::Overwrite(event))
        })
        .await
    }
}

pub enum OverwriteLoginIdEvent {
    Success,
    Invalid(ValidateOverwriteLoginIdFieldsError),
    NotFound,
    AlreadyRegistered,
    RepositoryError(RepositoryError),
}

mod overwrite_login_id_event {
    use super::OverwriteLoginIdEvent;

    const SUCCESS: &'static str = "overwrite login-id success";
    const ERROR: &'static str = "overwrite login-id error";

    impl std::fmt::Display for OverwriteLoginIdEvent {
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

async fn overwrite_login_id<S>(
    infra: &impl OverwriteLoginIdMaterial,
    fields: OverwriteLoginIdFieldsExtract,
    post: impl Fn(OverwriteLoginIdEvent) -> S,
) -> MethodResult<S> {
    let fields = OverwriteLoginIdFields::convert(fields)
        .map_err(|err| post(OverwriteLoginIdEvent::Invalid(err)))?;

    let login_id_repository = infra.login_id_repository();

    if login_id_repository
        .check_login_id_registered(&fields.new_login_id)
        .await
        .map_err(|err| post(OverwriteLoginIdEvent::RepositoryError(err)))?
    {
        return Err(post(OverwriteLoginIdEvent::AlreadyRegistered));
    }

    let user = login_id_repository
        .lookup_user(&fields.login_id)
        .await
        .map_err(|err| post(OverwriteLoginIdEvent::RepositoryError(err)))?
        .ok_or_else(|| post(OverwriteLoginIdEvent::NotFound))?;

    login_id_repository
        .overwrite_login_id(fields.new_login_id, user)
        .await
        .map_err(|err| post(OverwriteLoginIdEvent::RepositoryError(err)))?;

    Ok(post(OverwriteLoginIdEvent::Success))
}
