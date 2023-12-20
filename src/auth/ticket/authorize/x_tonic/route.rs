use std::sync::Arc;

use tonic::{metadata::MetadataMap, Request, Response, Status};

use crate::auth::ticket::authorize::y_protobuf::service::{
    authorize_pb_server::{AuthorizePb, AuthorizePbServer},
    AuthorizeRequestPb, AuthorizeResponsePb,
};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    common::api::{
        feature::AsInfra, logger::detail::StdoutJsonLogger, request::x_tonic::metadata::metadata,
        response::x_tonic::ServiceResponder,
    },
    x_content::{metadata::METADATA_AUTHORIZE_TOKEN, permission::AuthPermission},
};

use crate::auth::ticket::authorize::action::AuthorizeAction;

use crate::auth::ticket::authorize::infra::{AuthorizeFields, AuthorizeFieldsExtract};

use crate::{
    auth::ticket::{
        authorize::data::{AuthorizeSuccess, ValidateAuthorizeFieldsError},
        kernel::data::{
            AuthPermissionRequired, AuthorizeToken, ValidateAuthPermissionError,
            ValidateAuthorizeTokenError,
        },
    },
    common::api::request::data::RequestInfo,
};

#[derive(Default)]
pub struct ServiceAuthorize;

impl ServiceAuthorize {
    pub fn server(&self) -> AuthorizePbServer<Self> {
        AuthorizePbServer::new(ServiceAuthorize)
    }
}

#[async_trait::async_trait]
impl AuthorizePb for ServiceAuthorize {
    async fn authorize(
        &self,
        request: Request<AuthorizeRequestPb>,
    ) -> Result<Response<AuthorizeResponsePb>, Status> {
        async {
            let feature = AuthAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info));

            AuthorizeAction::live(feature.as_infra())
                .with_logger(logger)
                .authorize(request)
                .await
        }
        .await
        .respond_to()
    }
}

impl AuthorizeFieldsExtract for Request<AuthorizeRequestPb> {
    fn convert(self) -> Result<AuthorizeFields, ValidateAuthorizeFieldsError> {
        Ok(AuthorizeFields {
            token: decode_token(self.metadata()).map_err(ValidateAuthorizeFieldsError::Token)?,
            required: decode_permission_required(self.into_inner())
                .map_err(ValidateAuthorizeFieldsError::Required)?,
        })
    }
}

fn decode_token(map: &MetadataMap) -> Result<AuthorizeToken, ValidateAuthorizeTokenError> {
    Ok(AuthorizeToken::restore(
        metadata(map, METADATA_AUTHORIZE_TOKEN)
            .map_err(ValidateAuthorizeTokenError::MetadataError)?
            .ok_or(ValidateAuthorizeTokenError::NotFound)?
            .to_owned(),
    ))
}

fn decode_permission_required(
    request: AuthorizeRequestPb,
) -> Result<AuthPermissionRequired, ValidateAuthPermissionError> {
    if request.require_nothing {
        Ok(AuthPermissionRequired::Nothing)
    } else {
        let mut permissions = vec![];
        for permission in request.require_permissions {
            permissions.push(
                AuthPermission::convert(permission).ok_or(ValidateAuthPermissionError::Invalid)?,
            );
        }
        Ok(AuthPermissionRequired::HasSome(
            permissions.into_iter().collect(),
        ))
    }
}

impl ServiceResponder<AuthorizeResponsePb> for AuthorizeSuccess {
    fn respond_to(self) -> Result<Response<AuthorizeResponsePb>, Status> {
        let attrs = self.extract();
        Ok(Response::new(AuthorizeResponsePb {
            user_id: attrs.user_id.extract(),
            granted: Some(attrs.granted.into()),
        }))
    }
}

impl<R> ServiceResponder<R> for ValidateAuthorizeFieldsError {
    fn respond_to(self) -> Result<Response<R>, Status> {
        match self {
            Self::Token(err) => err.respond_to(),
            Self::Required(err) => err.respond_to(),
        }
    }
}
