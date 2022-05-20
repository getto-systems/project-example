use prost::Message;
use tonic::Request;

use crate::auth::user::account::unregister::y_protobuf::service::{
    unregister_auth_user_account_pb_client::UnregisterAuthUserAccountPbClient,
    UnregisterAuthUserAccountRequestPb,
};

use crate::auth::x_outside_feature::feature::AuthOutsideService;

use crate::auth::user::account::unregister::x_tonic::route::ServiceUnregisterUser;

use crate::z_lib::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{
    auth::proxy::helper::{proxy_infra_error, set_metadata},
    z_lib::{
        message::helper::{decode_base64, encode_protobuf_base64, invalid_protobuf},
        service::helper::new_endpoint,
    },
};

use crate::auth::{proxy::infra::AuthProxyService, ticket::validate::infra::AuthMetadataContent};

use crate::{
    auth::proxy::data::{AuthProxyError, AuthProxyResponse},
    z_lib::message::data::MessageError,
};

pub struct UnregisterUserProxyService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer<'a>,
    body: String,
}

impl<'a> UnregisterUserProxyService<'a> {
    pub fn new(service: &'a AuthOutsideService, request_id: &'a str, body: String) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(&service.google_authorizer),
            body,
        }
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyService for UnregisterUserProxyService<'a> {
    type Response = AuthProxyResponse;

    fn name(&self) -> &str {
        ServiceUnregisterUser::name()
    }
    async fn call(self, metadata: AuthMetadataContent) -> Result<Self::Response, AuthProxyError> {
        call(self, metadata).await
    }
}

async fn call<'a>(
    service: UnregisterUserProxyService<'a>,
    metadata: AuthMetadataContent,
) -> Result<AuthProxyResponse, AuthProxyError> {
    let mut client = UnregisterAuthUserAccountPbClient::new(
        new_endpoint(service.service_url)
            .map_err(|err| proxy_infra_error("service endpoint error", err))?
            .connect()
            .await
            .map_err(|err| proxy_infra_error("connect error", err))?,
    );

    let mut request =
        Request::new(decode_request(service.body).map_err(AuthProxyError::MessageError)?);
    set_metadata(
        &mut request,
        service.request_id,
        &service.authorizer,
        metadata,
    )
    .await
    .map_err(|err| proxy_infra_error("metadata error", err))?;

    let response = client
        .unregister_user(request)
        .await
        .map_err(AuthProxyError::from)?
        .into_inner();

    Ok(AuthProxyResponse::new(
        encode_protobuf_base64(response).map_err(AuthProxyError::MessageError)?,
    ))
}

fn decode_request(body: String) -> Result<UnregisterAuthUserAccountRequestPb, MessageError> {
    UnregisterAuthUserAccountRequestPb::decode(decode_base64(body)?).map_err(invalid_protobuf)
}
