use getto_application::data::MethodResult;

use crate::auth::ticket::remote::validate::method::validate_auth_token;

use crate::auth::user::account::remote::search::infra::{
    SearchUserAccountFieldsExtract, SearchUserAccountInfra, SearchUserAccountRepository,
};

use super::event::SearchUserAccountEvent;

use crate::auth::user::remote::kernel::data::RequireAuthRoles;

pub async fn search_user_account<S>(
    infra: &impl SearchUserAccountInfra,
    fields: SearchUserAccountFieldsExtract,
    post: impl Fn(SearchUserAccountEvent) -> S,
) -> MethodResult<S> {
    validate_auth_token(
        infra.validate_infra(),
        RequireAuthRoles::manage_auth_user(),
        |event| post(SearchUserAccountEvent::Validate(event)),
    )
    .await?;

    let fields = fields.into();

    let search_repository = infra.search_repository();
    let response = search_repository
        .search(&fields)
        .await
        .map_err(|err| post(SearchUserAccountEvent::RepositoryError(err)))?;

    Ok(post(SearchUserAccountEvent::Success(response)))
}
