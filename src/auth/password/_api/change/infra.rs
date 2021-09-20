use crate::auth::{
    auth_ticket::_common::kernel::infra::{AuthMetadata, AuthMetadataContent},
    password::_common::change::infra::ChangePasswordFieldsExtract,
};

use crate::{
    auth::{
        _common::service::data::AuthServiceError,
        password::_api::change::data::ChangePasswordResult,
    },
    z_details::_api::message::data::MessageError,
};

pub trait ChangePasswordInfra {
    type AuthMetadata: AuthMetadata;
    type ChangeService: ChangePasswordService;
    type ResponseEncoder: ChangePasswordResponseEncoder;

    fn auth_metadata(&self) -> &Self::AuthMetadata;
    fn change_service(&self) -> &Self::ChangeService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
}

pub trait ChangePasswordRequestDecoder {
    fn decode(self) -> Result<ChangePasswordFieldsExtract, MessageError>;
}

#[async_trait::async_trait]
pub trait ChangePasswordService {
    async fn change(
        &self,
        metadata: AuthMetadataContent,
        fields: ChangePasswordFieldsExtract,
    ) -> Result<ChangePasswordResponse, AuthServiceError>;
}

pub enum ChangePasswordResponse {
    Success,
    InvalidPassword,
}

pub trait ChangePasswordResponseEncoder {
    fn encode(
        &self,
        response: ChangePasswordResponse,
    ) -> Result<ChangePasswordResult, MessageError>;
}
