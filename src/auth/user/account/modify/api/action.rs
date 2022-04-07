use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{
    validate_auth_token, ValidateAuthTokenEvent, ValidateAuthTokenInfra,
};

use crate::auth::user::account::modify::infra::{
    ModifyAuthUserAccountFields, ModifyAuthUserAccountRepository,
    ModifyAuthUserAccountRequestDecoder,
};

use crate::{
    auth::user::account::modify::data::ValidateModifyAuthUserAccountFieldsError,
    z_lib::repository::data::RepositoryError,
};

pub enum ModifyAuthUserAccountState {
    Validate(ValidateAuthTokenEvent),
    ModifyUser(ModifyAuthUserAccountEvent),
}

impl std::fmt::Display for ModifyAuthUserAccountState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => event.fmt(f),
            Self::ModifyUser(event) => event.fmt(f),
        }
    }
}

pub trait ModifyAuthUserAccountMaterial {
    type Validate: ValidateAuthTokenInfra;

    type UserRepository: ModifyAuthUserAccountRepository;

    fn validate(&self) -> &Self::Validate;

    fn user_repository(&self) -> &Self::UserRepository;
}

pub struct ModifyAuthUserAccountAction<
    R: ModifyAuthUserAccountRequestDecoder,
    M: ModifyAuthUserAccountMaterial,
> {
    pubsub: ActionStatePubSub<ModifyAuthUserAccountState>,
    request_decoder: R,
    material: M,
}

impl<R: ModifyAuthUserAccountRequestDecoder, M: ModifyAuthUserAccountMaterial>
    ModifyAuthUserAccountAction<R, M>
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
        handler: impl 'static + Fn(&ModifyAuthUserAccountState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<ModifyAuthUserAccountState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = self.request_decoder.decode();

        validate_auth_token(m.validate(), |event| {
            pubsub.post(ModifyAuthUserAccountState::Validate(event))
        })
        .await?;

        modify_user(&m, fields, |event| {
            pubsub.post(ModifyAuthUserAccountState::ModifyUser(event))
        })
        .await
    }
}

pub enum ModifyAuthUserAccountEvent {
    Success,
    Invalid(ValidateModifyAuthUserAccountFieldsError),
    NotFound,
    Conflict,
    RepositoryError(RepositoryError),
}

mod modify_auth_user_account_event {
    use super::ModifyAuthUserAccountEvent;

    const SUCCESS: &'static str = "modify auth user account success";
    const ERROR: &'static str = "modify auth user account error";

    impl std::fmt::Display for ModifyAuthUserAccountEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Success => write!(f, "{}", SUCCESS),
                Self::Invalid(err) => err.fmt(f),
                Self::NotFound => write!(f, "{}; not found", ERROR),
                Self::Conflict => write!(f, "{}; changes conflicted", ERROR),
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

async fn modify_user<S>(
    infra: &impl ModifyAuthUserAccountMaterial,
    fields: Result<ModifyAuthUserAccountFields, ValidateModifyAuthUserAccountFieldsError>,
    post: impl Fn(ModifyAuthUserAccountEvent) -> S,
) -> MethodResult<S> {
    let fields = fields.map_err(|err| post(ModifyAuthUserAccountEvent::Invalid(err)))?;

    let user_repository = infra.user_repository();

    let user_id = user_repository
        .lookup_user_id(&fields.login_id)
        .await
        .map_err(|err| post(ModifyAuthUserAccountEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ModifyAuthUserAccountEvent::NotFound))?;

    let stored_user = user_repository
        .lookup_changes(&user_id)
        .await
        .map_err(|err| post(ModifyAuthUserAccountEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ModifyAuthUserAccountEvent::NotFound))?;

    if stored_user != fields.from {
        return Err(post(ModifyAuthUserAccountEvent::Conflict));
    }

    user_repository
        .modify_user(user_id, fields.to)
        .await
        .map_err(|err| post(ModifyAuthUserAccountEvent::RepositoryError(err)))?;

    Ok(post(ModifyAuthUserAccountEvent::Success))
}
