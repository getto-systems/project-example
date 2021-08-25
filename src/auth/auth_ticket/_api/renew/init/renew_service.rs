use tonic::transport::{Channel, ClientTlsConfig};
use tonic::Request;
use url::Url;

use crate::auth::auth_ticket::_common::y_protobuf::service::{
    renew_auth_ticket_pb_client::RenewAuthTicketPbClient, RenewAuthTicketRequestPb,
};

use crate::auth::_api::x_outside_feature::feature::AuthOutsideService;

use crate::auth::_api::service::helper::{infra_error, set_metadata};

use crate::auth::auth_ticket::_api::renew::infra::RenewAuthTicketService;

use crate::auth::{
    _api::service::data::AuthServiceError,
    auth_ticket::_common::{
        encode::data::AuthTicketEncoded,
        kernel::data::{AuthNonce, AuthToken},
    },
};

pub struct TonicRenewAuthTicketService<'a> {
    service_url: &'static str,
    request_id: &'a str,
}

impl<'a> TonicRenewAuthTicketService<'a> {
    pub const fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
        }
    }
}

#[async_trait::async_trait]
impl<'a> RenewAuthTicketService for TonicRenewAuthTicketService<'a> {
    async fn renew(
        &self,
        nonce: Option<AuthNonce>,
        token: Option<AuthToken>,
    ) -> Result<AuthTicketEncoded, AuthServiceError> {
        let mut client = RenewAuthTicketPbClient::new(self.new_channel().await?);

        let mut request = Request::new(RenewAuthTicketRequestPb {});
        set_metadata(&mut request, self.request_id, nonce, token)?;

        let response = client
            .renew(request)
            .await
            .map_err(AuthServiceError::from)?;

        let ticket: Option<AuthTicketEncoded> = response.into_inner().into();
        ticket.ok_or(AuthServiceError::InfraError(
            "failed to decode response".into(),
        ))
    }
}

impl<'a> TonicRenewAuthTicketService<'a> {
    async fn new_channel(&self) -> Result<Channel, AuthServiceError> {
        let url = Url::parse(self.service_url).map_err(infra_error)?;
        if url.scheme() == "https" {
            let config = ClientTlsConfig::new().domain_name(
                url.host_str()
                    .ok_or(AuthServiceError::InfraError("invalid service url".into()))?,
            );
            Channel::from_static(self.service_url)
                .tls_config(config)
                .map_err(infra_error)?
                .connect()
                .await
                .map_err(infra_error)
        } else {
            Channel::from_static(self.service_url)
                .connect()
                .await
                .map_err(infra_error)
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use crate::auth::auth_ticket::_api::renew::infra::RenewAuthTicketService;

    use crate::auth::{
        _api::service::data::AuthServiceError,
        auth_ticket::_common::{
            encode::data::AuthTicketEncoded,
            kernel::data::{AuthNonce, AuthToken, AuthTokenEncoded, AuthTokenExtract},
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
            _nonce: Option<AuthNonce>,
            _token: Option<AuthToken>,
        ) -> Result<AuthTicketEncoded, AuthServiceError> {
            Ok(AuthTicketEncoded {
                user: self.user.clone().extract(),
                token: AuthTokenEncoded {
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
            })
        }
    }
}
