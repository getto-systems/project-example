use tonic::Request;

use crate::auth::user::account::remote::y_protobuf::service::{
    search_user_account_pb_client::SearchUserAccountPbClient, SearchUserAccountRequestPb,
};

use crate::auth::remote::x_outside_feature::common::feature::AuthOutsideService;

use crate::z_lib::remote::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{
    auth::{
        remote::service::helper::{infra_error, set_metadata},
        user::account::remote::search::infra::SearchUserAccountFieldsExtract,
    },
    z_lib::remote::service::helper::new_endpoint,
};

use crate::{
    auth::{
        remote::service::proxy::AuthProxyService,
        ticket::remote::kernel::infra::AuthMetadataContent,
        user::account::remote::proxy_search::infra::SearchUserAccountProxyResponse,
    },
    z_lib::remote::service::infra::ServiceAuthorizer,
};

use crate::auth::remote::service::data::AuthServiceError;

pub struct ProxyService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer,
}

impl<'a> ProxyService<'a> {
    pub fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(service.service_url),
        }
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyService<SearchUserAccountFieldsExtract, SearchUserAccountProxyResponse>
    for ProxyService<'a>
{
    fn name(&self) -> &str {
        "auth.user.account.search"
    }
    async fn call(
        &self,
        metadata: AuthMetadataContent,
        params: SearchUserAccountFieldsExtract,
    ) -> Result<SearchUserAccountProxyResponse, AuthServiceError> {
        let mut client = SearchUserAccountPbClient::new(
            new_endpoint(self.service_url)
                .map_err(infra_error)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let mut request = Request::new(SearchUserAccountRequestPb {
            offset: params.offset,
            sort_key: params.sort.key,
            sort_order: params.sort.order,
            login_id: params.login_id,
        });
        set_metadata(
            &mut request,
            self.request_id,
            self.authorizer.fetch_token().await.map_err(infra_error)?,
            metadata,
        )
        .map_err(infra_error)?;

        let response = client
            .search(request)
            .await
            .map_err(AuthServiceError::from)?;
        Ok(response.into_inner().into())
    }
}
