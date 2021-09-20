use tonic::Request;

use crate::auth::password::_common::y_protobuf::service::{
    authenticate_password_pb_client::AuthenticatePasswordPbClient, AuthenticatePasswordRequestPb,
};

use crate::auth::_common::x_outside_feature::feature::AuthOutsideService;

use crate::z_details::_common::service::init::authorizer::GoogleServiceAuthorizer;

use crate::auth::_common::service::helper::{
    infra_error, new_endpoint, set_authorization, set_metadata,
};

use crate::auth::{
    auth_ticket::_common::kernel::infra::AuthMetadataContent,
    password::{
        _api::authenticate::infra::{AuthenticatePasswordResponse, AuthenticatePasswordService},
        _common::authenticate::infra::AuthenticatePasswordFieldsExtract,
    },
};

use crate::auth::_common::service::data::AuthServiceError;

pub struct TonicAuthenticatePasswordService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer,
}

impl<'a> TonicAuthenticatePasswordService<'a> {
    pub fn new(service: &'a AuthOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(service.service_url),
        }
    }
}

#[async_trait::async_trait]
impl<'a> AuthenticatePasswordService for TonicAuthenticatePasswordService<'a> {
    async fn authenticate(
        &self,
        metadata: AuthMetadataContent,
        fields: AuthenticatePasswordFieldsExtract,
    ) -> Result<AuthenticatePasswordResponse, AuthServiceError> {
        let mut client = AuthenticatePasswordPbClient::new(
            new_endpoint(self.service_url)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let mut request = Request::new(AuthenticatePasswordRequestPb {
            login_id: fields.login_id,
            password: fields.password,
        });
        set_authorization(&mut request, &self.authorizer).await?;
        set_metadata(&mut request, self.request_id, metadata)?;

        let response = client
            .authenticate(request)
            .await
            .map_err(AuthServiceError::from)?;
        let response: Option<AuthenticatePasswordResponse> = response.into_inner().into();
        response.ok_or(AuthServiceError::InfraError(
            "failed to decode response".into(),
        ))
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use crate::auth::{
        auth_ticket::_common::kernel::infra::AuthMetadataContent,
        password::{
            _api::authenticate::infra::{
                AuthenticatePasswordResponse, AuthenticatePasswordService,
            },
            _common::authenticate::infra::AuthenticatePasswordFieldsExtract,
        },
    };

    use crate::auth::{
        _common::service::data::AuthServiceError,
        auth_ticket::_common::{
            encode::data::AuthTicketEncoded,
            kernel::data::{AuthTokenEncoded, AuthTokenExtract},
        },
        auth_user::_common::kernel::data::AuthUser,
    };

    pub struct StaticAuthenticatePasswordService {
        pub user: AuthUser,
    }

    #[async_trait::async_trait]
    impl AuthenticatePasswordService for StaticAuthenticatePasswordService {
        async fn authenticate(
            &self,
            _metadata: AuthMetadataContent,
            _fields: AuthenticatePasswordFieldsExtract,
        ) -> Result<AuthenticatePasswordResponse, AuthServiceError> {
            Ok(AuthenticatePasswordResponse::Success(AuthTicketEncoded {
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
            }))
        }
    }
}
