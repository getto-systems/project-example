use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::auth::user::account::{
    search::y_protobuf::service::{
        search_auth_user_account_pb_server::SearchAuthUserAccountPb,
        SearchAuthUserAccountRequestPb, SearchAuthUserAccountResponsePb,
    },
    y_protobuf::service::AuthUserAccountPb,
};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    common::api::{
        feature::AsInfra, logger::detail::StdoutJsonLogger, response::x_tonic::ServiceResponder,
    },
    x_content::permission::AuthPermission,
};

use crate::auth::{
    ticket::authorize::action::AuthorizeAction,
    user::account::search::action::SearchAuthUserAccountAction,
};

use crate::auth::user::account::search::infra::SearchAuthUserAccountFilterExtract;

use crate::{
    auth::{
        ticket::authorize::data::AuthorizeError,
        user::{
            account::search::data::{
                AuthUserAccountSearch, SearchAuthUserAccountFilter,
                SearchAuthUserAccountFilterProps,
            },
            login_id::kernel::data::SearchLoginId,
        },
    },
    common::api::{
        repository::data::RepositoryError, request::data::RequestInfo, search::data::SearchProps,
    },
};

pub struct ServiceSearch;

#[async_trait::async_trait]
impl SearchAuthUserAccountPb for ServiceSearch {
    async fn search(
        &self,
        request: Request<SearchAuthUserAccountRequestPb>,
    ) -> Result<Response<SearchAuthUserAccountResponsePb>, Status> {
        async {
            let feature = AuthAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info));

            let (infra, _) = AuthorizeAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .pick_authorized_infra(&feature, request.metadata())
                .await?;

            Ok::<_, AppError>(
                SearchAuthUserAccountAction::live(infra)
                    .with_logger(logger.clone())
                    .search(request.into_inner())
                    .await?,
            )
        }
        .await
        .respond_to()
    }
}

enum AppError {
    AuthorizeError(AuthorizeError),
    RepositoryError(RepositoryError),
}

impl From<AuthorizeError> for AppError {
    fn from(value: AuthorizeError) -> Self {
        Self::AuthorizeError(value)
    }
}

impl From<RepositoryError> for AppError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

impl SearchAuthUserAccountFilterExtract for SearchAuthUserAccountRequestPb {
    fn convert(self) -> SearchAuthUserAccountFilter {
        let mut filter = self.filter.unwrap_or_default();
        SearchAuthUserAccountFilter {
            search: SearchProps {
                offset: self.offset.into(),
                sort: self.sort.into(),
            },
            props: SearchAuthUserAccountFilterProps {
                login_id: SearchLoginId::restore(filter.login_id.pop()),
                granted: filter
                    .granted
                    .into_iter()
                    .filter_map(AuthPermission::convert)
                    .collect(),
            },
        }
    }
}

impl ServiceResponder<SearchAuthUserAccountResponsePb> for AppError {
    fn respond_to(self) -> Result<Response<SearchAuthUserAccountResponsePb>, Status> {
        match self {
            Self::AuthorizeError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<SearchAuthUserAccountResponsePb> for AuthUserAccountSearch {
    fn respond_to(self) -> Result<Response<SearchAuthUserAccountResponsePb>, Status> {
        Ok(Response::new(SearchAuthUserAccountResponsePb {
            page: Some(self.page.into()),
            sort: Some(self.sort.into()),
            users: self
                .users
                .into_iter()
                .map(|user| AuthUserAccountPb {
                    login_id: user.login_id.extract(),
                    granted: user.attrs.granted.extract().into_iter().collect(),
                    memo: user.attrs.memo.extract(),
                    reset_token_destination: Some(user.reset_token_destination.into()),
                })
                .collect(),
            ..Default::default()
        }))
    }
}
