use getto_application::data::MethodResult;

use crate::auth::ticket::remote::validate::method::validate_auth_token;

use crate::auth::user::account::remote::search::infra::{
    SearchAuthUserAccountFieldsExtract, SearchAuthUserAccountInfra, SearchAuthUserAccountRepository,
};

use super::event::SearchAuthUserAccountEvent;

use crate::auth::user::remote::kernel::data::RequireAuthRoles;

pub async fn search_user_account<S>(
    infra: &impl SearchAuthUserAccountInfra,
    fields: SearchAuthUserAccountFieldsExtract,
    post: impl Fn(SearchAuthUserAccountEvent) -> S,
) -> MethodResult<S> {
    validate_auth_token(
        infra.validate_infra(),
        RequireAuthRoles::Nothing, // TODO RequireAuthRoles::manage_auth_user(),
        |event| post(SearchAuthUserAccountEvent::Validate(event)),
    )
    .await?;

    let fields = fields.into();

    let search_repository = infra.search_repository();
    let response = search_repository
        .search(&fields)
        .await
        .map_err(|err| post(SearchAuthUserAccountEvent::RepositoryError(err)))?;

    Ok(post(SearchAuthUserAccountEvent::Success(response)))
}
