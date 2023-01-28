use tonic::metadata::MetadataMap;

use crate::auth::ticket::authorize::y_protobuf::service::ClarifyAuthorizeTokenRequestPb;

use crate::x_content::{metadata::METADATA_AUTHORIZE_TOKEN, permission::AuthPermission};

use crate::common::api::request::x_tonic::metadata::metadata;

use crate::auth::ticket::authorize::infra::{AuthorizeFields, AuthorizeFieldsExtract};

use crate::auth::ticket::{
    authorize::data::ValidateAuthorizeFieldsError,
    kernel::data::{
        AuthPermissionRequired, AuthorizeToken, ValidateAuthPermissionError,
        ValidateAuthorizeTokenError,
    },
};

impl AuthorizeFieldsExtract for (&MetadataMap, ClarifyAuthorizeTokenRequestPb) {
    fn convert(self) -> Result<AuthorizeFields, ValidateAuthorizeFieldsError> {
        Ok(AuthorizeFields {
            token: decode_token(self.0).map_err(ValidateAuthorizeFieldsError::Token)?,
            required: decode_permission_required(self.1)
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
    request: ClarifyAuthorizeTokenRequestPb,
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
