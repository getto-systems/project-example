use std::sync::Arc;

use crate::{
    auth::user::kernel::detail::repository::memory::{
        login_id::MapLoginId, user::MapUser, StoreLoginId, StoreUser,
    },
    common::api::feature::AsInfra,
    x_content::permission::AuthPermission,
};

use crate::auth::user::account::search::action::{
    SearchAuthUserAccountAction, SearchAuthUserAccountInfo,
};

use crate::auth::user::password::kernel::infra::HashedPassword;

use crate::{
    auth::{
        ticket::kernel::data::{AuthPermissionGranted, AuthPermissionRequired},
        user::{
            account::{
                kernel::data::AuthUserMemo,
                search::data::{SearchAuthUserAccountFilter, SearchAuthUserAccountFilterProps},
            },
            kernel::data::{AuthUser, AuthUserId},
            login_id::kernel::data::{LoginId, SearchLoginId},
        },
    },
    common::api::{repository::data::RepositoryError, search::data::SearchProps},
};

#[tokio::test]
async fn info() {
    assert_eq!(
        SearchAuthUserAccountInfo::required(),
        AuthPermissionRequired::user(),
    );
}

#[tokio::test]
async fn success() -> Result<(), RepositoryError> {
    let feature = feature();
    let action = SearchAuthUserAccountAction::mock(feature.as_infra());

    let filter = SearchAuthUserAccountFilter {
        search: SearchProps {
            offset: Default::default(),
            sort: Default::default(),
        },
        props: SearchAuthUserAccountFilterProps {
            login_id: SearchLoginId::restore(None),
            granted: vec![],
        },
    };

    action.search(filter).await?;

    Ok(())
}

fn feature() -> (Arc<StoreLoginId>, Arc<StoreUser>) {
    let login_id_store = Arc::new(StoreLoginId::default());
    let user_store = Arc::new(StoreUser::default());

    for (user, login_id, memo) in vec![(
        AuthUser {
            user_id: AuthUserId::restore("user-id".to_owned()),
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
        },
        LoginId::restore("login-id".to_owned()),
        Some(AuthUserMemo::restore("memo".to_owned())),
    )] {
        MapLoginId::insert_entry(
            &login_id_store,
            login_id.clone(),
            user.user_id.clone(),
            Default::default(),
        );
        MapUser::insert_entry(
            &user_store,
            user.user_id,
            (
                login_id,
                Some(user.granted),
                Some(HashedPassword::restore("PASSWORD".to_owned())),
                memo,
            ),
        );
    }

    (login_id_store, user_store)
}
