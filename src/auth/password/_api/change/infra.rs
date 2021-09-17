use crate::auth::{
    auth_ticket::_api::validate::infra::ValidateApiTokenInfra,
    auth_user::_common::kernel::data::AuthUserId,
    password::_common::change::infra::ChangePasswordFieldsExtract,
};

use crate::{
    auth::{
        _api::service::data::AuthServiceError, password::_api::change::data::ChangePasswordResult,
    },
    z_details::_api::message::data::MessageError,
};

pub trait ChangePasswordInfra {
    type ValidateInfra: ValidateApiTokenInfra;
    type ChangeService: ChangePasswordService;
    type ResponseEncoder: ChangePasswordResponseEncoder;

    fn validate_infra(&self) -> &Self::ValidateInfra;
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
        user_id: AuthUserId,
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
