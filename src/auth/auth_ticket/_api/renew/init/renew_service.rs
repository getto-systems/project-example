use tonic::metadata::MetadataValue;
use tonic::Request;

use crate::auth::auth_ticket::_common::y_protobuf::service::{
    renew_auth_ticket_pb_client::RenewAuthTicketPbClient, RenewAuthTicketRequestPb,
};

use crate::{
    auth::{
        _api::x_outside_feature::feature::AuthOutsideService,
        auth_ticket::_common::kernel::x_tonic::metadata::{METADATA_NONCE, METADATA_TICKET_TOKEN},
    },
    x_outside_feature::_common::metadata::METADATA_REQUEST_ID,
};

use crate::auth::_api::service::helper::infra_error;

use crate::auth::auth_ticket::_api::renew::infra::RenewAuthTicketService;

use crate::auth::{
    _api::service::data::ServiceError,
    auth_ticket::{
        _api::kernel::data::{AuthNonceValue, AuthTokenValue},
        _common::encode::data::EncodeAuthTicketResponse,
    },
};

pub struct TonicRenewAuthTicketService<'a> {
    auth_service_url: &'static str,
    request_id: &'a str,
}

impl<'a> TonicRenewAuthTicketService<'a> {
    pub const fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            auth_service_url: service.auth_service_url,
            request_id,
        }
    }
}

#[async_trait::async_trait]
impl<'a> RenewAuthTicketService for TonicRenewAuthTicketService<'a> {
    async fn renew(
        &self,
        nonce: AuthNonceValue,
        token: AuthTokenValue,
    ) -> Result<EncodeAuthTicketResponse, ServiceError> {
        let mut client = RenewAuthTicketPbClient::connect(self.auth_service_url)
            .await
            .map_err(infra_error)?;

        let mut request = Request::new(RenewAuthTicketRequestPb {});
        // TODO request_id と nonce と token を設定する helper が欲しい
        request.metadata_mut().append(
            METADATA_REQUEST_ID,
            MetadataValue::from_str(self.request_id).map_err(infra_error)?,
        );
        request.metadata_mut().append(
            METADATA_NONCE,
            MetadataValue::from_str(&nonce.extract()).map_err(infra_error)?,
        );
        request.metadata_mut().append(
            METADATA_TICKET_TOKEN,
            MetadataValue::from_str(&token.extract()).map_err(infra_error)?,
        );

        client
            .renew(request)
            .await
            .map_err(Into::into)
            .and_then(|response| {
                let response: Option<EncodeAuthTicketResponse> = response.into_inner().into();
                response.ok_or_else(|| ServiceError::InfraError("failed to decode response".into()))
            })
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use crate::auth::auth_ticket::_api::renew::infra::RenewAuthTicketService;

    use crate::auth::{
        _api::service::data::ServiceError,
        auth_ticket::{
            _api::kernel::data::{AuthNonceValue, AuthTokenValue},
            _common::{
                encode::data::EncodeAuthTicketResponse,
                kernel::data::{AuthTokenEncoded, AuthTokenExtract},
            },
        },
        auth_user::_common::kernel::data::AuthUser,
    };

    pub struct StaticRenewAuthTicketService {
        pub user: AuthUser,
    }

    #[async_trait::async_trait]
    impl RenewAuthTicketService for StaticRenewAuthTicketService {
        async fn renew(
            &self,
            _nonce: AuthNonceValue,
            _token: AuthTokenValue,
        ) -> Result<EncodeAuthTicketResponse, ServiceError> {
            Ok(EncodeAuthTicketResponse::new(
                self.user.clone(),
                AuthTokenEncoded {
                    ticket_token: AuthTokenExtract {
                        token: "TICKET-TOKEN".into(),
                        expires: 0,
                    },
                    api_token: AuthTokenExtract {
                        token: "API-TOKEN".into(),
                        expires: 0,
                    },
                    cloudfront_tokens: HashMap::new(),
                },
            ))
        }
    }
}
