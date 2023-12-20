use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::auth::user::password::authenticate::y_protobuf::service::{
    authenticate_with_password_pb_server::{
        AuthenticateWithPasswordPb, AuthenticateWithPasswordPbServer,
    },
    AuthenticateWithPasswordRequestPb, AuthenticateWithPasswordResponsePb,
};

use crate::{
    common::api::{
        feature::AsInfra, logger::detail::StdoutJsonLogger, response::x_tonic::ServiceResponder,
    },
    x_outside_feature::auth::feature::AuthAppFeature,
};

use crate::auth::{
    ticket::{encode::action::EncodeAuthTokenAction, issue::action::IssueAuthTicketAction},
    user::password::authenticate::action::AuthenticateWithPasswordAction,
};

use crate::auth::user::password::{
    authenticate::infra::{AuthenticateWithPasswordFields, AuthenticateWithPasswordFieldsExtract},
    kernel::infra::PlainPassword,
};

use crate::{
    auth::{
        ticket::{
            encode::data::{EncodeAuthTokenError, EncodeAuthTokenSuccess},
            issue::data::IssueAuthTicketError,
        },
        user::{
            login_id::kernel::data::LoginId,
            password::authenticate::data::{
                AuthenticateWithPasswordError, ValidateAuthenticateWithPasswordFieldsError,
            },
        },
    },
    common::api::request::data::RequestInfo,
};

#[derive(Default)]
pub struct ServiceAuthenticateWithPassword;

impl ServiceAuthenticateWithPassword {
    pub fn server(&self) -> AuthenticateWithPasswordPbServer<Self> {
        AuthenticateWithPasswordPbServer::new(Self)
    }
}

#[async_trait::async_trait]
impl AuthenticateWithPasswordPb for ServiceAuthenticateWithPassword {
    async fn authenticate(
        &self,
        request: Request<AuthenticateWithPasswordRequestPb>,
    ) -> Result<Response<AuthenticateWithPasswordResponsePb>, Status> {
        async {
            let feature = AuthAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info));

            let auth = AuthenticateWithPasswordAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .authenticate(request.into_inner())
                .await?;

            let ticket = IssueAuthTicketAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .issue(auth)
                .await?;

            Ok::<_, AppError>(
                EncodeAuthTokenAction::live(feature.as_infra())
                    .with_logger(logger)
                    .encode(ticket)
                    .await?,
            )
        }
        .await
        .respond_to()
    }
}

enum AppError {
    AuthenticateWithPasswordError(AuthenticateWithPasswordError),
    IssueAuthTicketError(IssueAuthTicketError),
    EncodeAuthTokenError(EncodeAuthTokenError),
}

impl From<AuthenticateWithPasswordError> for AppError {
    fn from(value: AuthenticateWithPasswordError) -> Self {
        Self::AuthenticateWithPasswordError(value)
    }
}

impl From<IssueAuthTicketError> for AppError {
    fn from(value: IssueAuthTicketError) -> Self {
        Self::IssueAuthTicketError(value)
    }
}

impl From<EncodeAuthTokenError> for AppError {
    fn from(value: EncodeAuthTokenError) -> Self {
        Self::EncodeAuthTokenError(value)
    }
}

impl AuthenticateWithPasswordFieldsExtract for AuthenticateWithPasswordRequestPb {
    fn convert(
        self,
    ) -> Result<AuthenticateWithPasswordFields, ValidateAuthenticateWithPasswordFieldsError> {
        Ok(AuthenticateWithPasswordFields {
            login_id: LoginId::convert(self.login_id)
                .map_err(ValidateAuthenticateWithPasswordFieldsError::InvalidLoginId)?,
            plain_password: PlainPassword::convert(self.password)
                .map_err(ValidateAuthenticateWithPasswordFieldsError::InvalidPassword)?,
        })
    }
}

impl ServiceResponder<AuthenticateWithPasswordResponsePb> for EncodeAuthTokenSuccess {
    fn respond_to(self) -> Result<Response<AuthenticateWithPasswordResponsePb>, Status> {
        let (token, granted) = self.extract();
        Ok(Response::new(AuthenticateWithPasswordResponsePb {
            success: true,
            token: Some(token.into()),
            granted: Some(granted.into()),
        }))
    }
}

impl ServiceResponder<AuthenticateWithPasswordResponsePb> for AuthenticateWithPasswordError {
    fn respond_to(self) -> Result<Response<AuthenticateWithPasswordResponsePb>, Status> {
        match self {
            Self::Invalid(_) => Ok(Response::new(AuthenticateWithPasswordResponsePb {
                success: false,
                ..Default::default()
            })),
            Self::NotFound(_login_id) => Ok(Response::new(AuthenticateWithPasswordResponsePb {
                success: false,
                ..Default::default()
            })),
            Self::PasswordNotMatched => Ok(Response::new(AuthenticateWithPasswordResponsePb {
                success: false,
                ..Default::default()
            })),
            Self::PasswordHashError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<AuthenticateWithPasswordResponsePb> for AppError {
    fn respond_to(self) -> Result<Response<AuthenticateWithPasswordResponsePb>, Status> {
        match self {
            Self::AuthenticateWithPasswordError(err) => err.respond_to(),
            Self::IssueAuthTicketError(err) => err.respond_to(),
            Self::EncodeAuthTokenError(err) => err.respond_to(),
        }
    }
}
