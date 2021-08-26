use tonic::Request;

use crate::auth::auth_ticket::_common::y_protobuf::service::{
    renew_auth_ticket_pb_client::RenewAuthTicketPbClient, RenewAuthTicketRequestPb,
};

use crate::auth::_api::x_outside_feature::feature::AuthOutsideService;

use crate::auth::_api::service::helper::{
    infra_error, new_endpoint, set_authorization, set_metadata, AuthAuthorizer,
};

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
    authorizer: AuthAuthorizer,
}

impl<'a> TonicRenewAuthTicketService<'a> {
    pub fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: AuthAuthorizer::new(service.service_url),
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
        let mut client = RenewAuthTicketPbClient::new(
            new_endpoint(self.service_url)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let mut request = Request::new(RenewAuthTicketRequestPb {});
        // TODO authorizer は別な infra として分離するべき
        set_authorization(&mut request, self.authorizer.fetch_token().await?)?;
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
