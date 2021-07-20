use tonic::{Response, Status};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::auth::auth_ticket::_common::y_protobuf::service::RenewAuthTicketResponsePb;

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
