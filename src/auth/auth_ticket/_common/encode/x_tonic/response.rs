use tonic::{Response, Status};

use crate::auth::password::reset::_common::y_protobuf::service::ResetPasswordResponsePb;
use crate::auth::{
    auth_ticket::_common::y_protobuf::service::RenewAuthTicketResponsePb,
    password::_common::y_protobuf::service::AuthenticatePasswordResponsePb,
};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::auth::{
    auth_ticket::_common::{
        encode::data::EncodeAuthTicketResponse, kernel::data::AuthTokenEncoded,
    },
    auth_user::_common::kernel::data::AuthUserExtract,
};

impl RespondTo<RenewAuthTicketResponsePb> for EncodeAuthTicketResponse {
    fn respond_to(self) -> Result<Response<RenewAuthTicketResponsePb>, Status> {
        let (user, token) = self.extract();
        Ok(Response::new(RenewAuthTicketResponsePb {
            user: Some(user.into()),
            token: Some(token.into()),
        }))
    }
}

impl RespondTo<AuthenticatePasswordResponsePb> for EncodeAuthTicketResponse {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        let (user, token) = self.extract();
        Ok(Response::new(AuthenticatePasswordResponsePb {
            success: true,
            user: Some(user.into()),
            token: Some(token.into()),
        }))
    }
}

impl RespondTo<ResetPasswordResponsePb> for EncodeAuthTicketResponse {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        let (user, token) = self.extract();
        Ok(Response::new(ResetPasswordResponsePb {
            success: true,
            user: Some(user.into()),
            token: Some(token.into()),
            ..Default::default()
        }))
    }
}

impl Into<Option<EncodeAuthTicketResponse>> for RenewAuthTicketResponsePb {
    fn into(self) -> Option<EncodeAuthTicketResponse> {
        match (self.user, self.token) {
            (Some(user), Some(token)) => {
                let user: AuthUserExtract = user.into();
                let token: Option<AuthTokenEncoded> = token.into();
                token.map(|token| EncodeAuthTicketResponse::new(user.into(), token))
            }
            _ => None,
        }
    }
}
