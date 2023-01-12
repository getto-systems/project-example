use tonic::Request;

use crate::auth::ticket::authorize::y_protobuf::service::{
    clarify_authorize_token_pb_client::ClarifyAuthorizeTokenPbClient,
    ClarifyAuthorizeTokenRequestPb,
};

use crate::{
    common::x_outside_feature::feature::CommonOutsideService, x_outside_feature::data::RequestId,
};

use crate::common::api::service::init::service::GoogleTonicService;

use crate::common::proxy::infra::ProxyCall;

use crate::{
    auth::{
        proxy::data::AuthProxyError, ticket::kernel::data::AuthPermissionRequired,
        user::kernel::data::AuthUserId,
    },
    common::proxy::data::ProxyMetadataExtract,
};

pub struct TonicClarifyAuthorizeTokenProxyCall<'a> {
    service: GoogleTonicService<'a>,
}

impl<'a> TonicClarifyAuthorizeTokenProxyCall<'a> {
    pub fn new(service: &'a CommonOutsideService, request_id: RequestId) -> Self {
        Self {
            service: GoogleTonicService::new(service, request_id),
        }
    }
}

#[async_trait::async_trait]
impl<'a> ProxyCall for TonicClarifyAuthorizeTokenProxyCall<'a> {
    type Request = AuthPermissionRequired;
    type Response = AuthUserId;
    type Error = AuthProxyError;

    async fn call(
        &self,
        metadata: impl ProxyMetadataExtract,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let mut client = ClarifyAuthorizeTokenPbClient::new(
            self.service
                .endpoint()
                .await
                .map_err(AuthProxyError::ServiceConnectError)?,
        );

        let mut request = Request::new(match request {
            AuthPermissionRequired::Nothing => ClarifyAuthorizeTokenRequestPb {
                require_nothing: true,
                ..Default::default()
            },
            AuthPermissionRequired::HasSome(permissions) => ClarifyAuthorizeTokenRequestPb {
                require_permissions: permissions
                    .into_iter()
                    .map(|permission| permission.extract())
                    .collect(),
                ..Default::default()
            },
        });
        self.service
            .set_metadata(&mut request, metadata)
            .await
            .map_err(AuthProxyError::ServiceMetadataError)?;

        let response = client
            .clarify(request)
            .await
            .map_err(AuthProxyError::from)?;

        let user_id = response.into_inner().user_id;

        Ok(AuthUserId::restore(user_id))
    }
}

#[cfg(test)]
pub mod test {
    use crate::common::proxy::infra::ProxyCall;

    use crate::{
        auth::{
            proxy::data::AuthProxyError, ticket::kernel::data::AuthPermissionRequired,
            user::kernel::data::AuthUserId,
        },
        common::proxy::data::ProxyMetadataExtract,
    };

    pub struct StaticClarifyAuthorizeTokenProxyCall(AuthUserId);

    impl StaticClarifyAuthorizeTokenProxyCall {
        pub fn new(user_id: AuthUserId) -> Self {
            Self(user_id)
        }
    }

    #[async_trait::async_trait]
    impl ProxyCall for StaticClarifyAuthorizeTokenProxyCall {
        type Request = AuthPermissionRequired;
        type Response = AuthUserId;
        type Error = AuthProxyError;

        async fn call(
            &self,
            _metadata: impl ProxyMetadataExtract,
            _request: Self::Request,
        ) -> Result<Self::Response, Self::Error> {
            Ok(self.0.clone())
        }
    }
}
