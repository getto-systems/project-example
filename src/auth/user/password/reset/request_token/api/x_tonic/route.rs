use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::auth::user::password::reset::request_token::y_protobuf::service::{
    request_reset_token_pb_server::{RequestResetTokenPb, RequestResetTokenPbServer},
    RequestResetTokenRequestPb, RequestResetTokenResponsePb,
};

use crate::{
    common::api::response::x_tonic::ServiceResponder,
    x_outside_feature::auth::feature::AuthAppFeature,
};

use crate::common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger};

use crate::auth::user::password::reset::request_token::action::RequestResetPasswordTokenAction;

use crate::auth::user::password::reset::request_token::infra::{
    RequestResetPasswordTokenFields, RequestResetPasswordTokenFieldsExtract,
};

use crate::{
    auth::user::{
        login_id::kernel::data::{LoginId, ValidateLoginIdError},
        password::reset::request_token::data::{
            EncodeResetTokenError, NotifyResetTokenError, NotifyResetTokenResponse,
            RequestResetPasswordTokenError,
        },
    },
    common::api::request::data::RequestInfo,
};

#[derive(Default)]
pub struct ServiceRequestToken;

impl ServiceRequestToken {
    pub fn server(&self) -> RequestResetTokenPbServer<Self> {
        RequestResetTokenPbServer::new(Self)
    }
}

#[async_trait::async_trait]
impl RequestResetTokenPb for ServiceRequestToken {
    async fn request_token(
        &self,
        request: Request<RequestResetTokenRequestPb>,
    ) -> Result<Response<RequestResetTokenResponsePb>, Status> {
        async {
            let feature = AuthAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info));

            RequestResetPasswordTokenAction::live(feature.as_infra())
                .with_logger(logger)
                .request(request.into_inner())
                .await
        }
        .await
        .respond_to()
    }
}

impl RequestResetPasswordTokenFieldsExtract for RequestResetTokenRequestPb {
    fn convert(self) -> Result<RequestResetPasswordTokenFields, ValidateLoginIdError> {
        Ok(RequestResetPasswordTokenFields {
            login_id: LoginId::convert(self.login_id)?,
        })
    }
}

impl ServiceResponder<RequestResetTokenResponsePb> for NotifyResetTokenResponse {
    fn respond_to(self) -> Result<Response<RequestResetTokenResponsePb>, Status> {
        Ok(Response::new(RequestResetTokenResponsePb { success: true }))
    }
}

impl ServiceResponder<RequestResetTokenResponsePb> for RequestResetPasswordTokenError {
    fn respond_to(self) -> Result<Response<RequestResetTokenResponsePb>, Status> {
        match self {
            Self::Invalid(_) => Ok(Response::new(RequestResetTokenResponsePb {
                success: false,
            })),
            Self::NotFound => Ok(Response::new(RequestResetTokenResponsePb {
                success: false,
            })),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
            Self::NotifyError(err) => err.respond_to(),
        }
    }
}

impl<T> ServiceResponder<T> for EncodeResetTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::InfraError(_) => Err(Status::internal("encode reset token error")),
        }
    }
}

impl<T> ServiceResponder<T> for NotifyResetTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::NoDestination => Err(Status::internal("no reset token destination")),
            Self::NotificationError(_) => Err(Status::internal("notify reset token error")),
        }
    }
}
