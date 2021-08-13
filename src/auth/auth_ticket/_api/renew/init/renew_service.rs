use tonic::Request;

use crate::auth::auth_ticket::_common::y_protobuf::service::{
    renew_auth_ticket_pb_client::RenewAuthTicketPbClient, RenewAuthTicketRequestPb,
};

use crate::auth::_api::x_outside_feature::feature::AuthOutsideService;

use crate::auth::_api::service::helper::{infra_error, set_metadata};

use crate::auth::auth_ticket::_api::renew::infra::RenewAuthTicketService;

use crate::auth::{
    _api::service::data::ServiceError,
    auth_ticket::_common::{
        encode::data::EncodeAuthTicketResponse,
        kernel::data::{AuthNonceValue, AuthTokenValue},
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
        set_metadata(&mut request, self.request_id, nonce, token)?;

        let response = client.renew(request).await.map_err(ServiceError::from)?;
        let response: Option<EncodeAuthTicketResponse> = response.into_inner().into();
        response.ok_or(ServiceError::InfraError("failed to decode response".into()))
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use crate::auth::auth_ticket::_api::renew::infra::RenewAuthTicketService;

    use crate::auth::{
        _api::service::data::ServiceError,
        auth_ticket::_common::{
            encode::data::EncodeAuthTicketResponse,
            kernel::data::{AuthNonceValue, AuthTokenEncoded, AuthTokenExtract, AuthTokenValue},
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
