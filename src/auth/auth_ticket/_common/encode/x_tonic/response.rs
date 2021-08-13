use tonic::{Response, Status};

use crate::auth::password::reset::_common::y_protobuf::service::ResetPasswordResponsePb;
use crate::auth::{
    auth_ticket::_common::y_protobuf::service::RenewAuthTicketResponsePb,
    password::_common::y_protobuf::service::AuthenticatePasswordResponsePb,
};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::auth::{
    auth_ticket::_common::{encode::data::AuthTicketEncoded, kernel::data::AuthTokenEncoded},
    auth_user::_common::kernel::data::AuthUserExtract,
};

impl RespondTo<RenewAuthTicketResponsePb> for AuthTicketEncoded {
    fn respond_to(self) -> Result<Response<RenewAuthTicketResponsePb>, Status> {
        Ok(Response::new(RenewAuthTicketResponsePb {
            user: Some(self.user.into()),
            token: Some(self.token.into()),
        }))
    }
}

impl RespondTo<AuthenticatePasswordResponsePb> for AuthTicketEncoded {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        Ok(Response::new(AuthenticatePasswordResponsePb {
            success: true,
            user: Some(self.user.into()),
            token: Some(self.token.into()),
        }))
    }
}

impl RespondTo<ResetPasswordResponsePb> for AuthTicketEncoded {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        Ok(Response::new(ResetPasswordResponsePb {
            success: true,
            user: Some(self.user.into()),
            token: Some(self.token.into()),
            ..Default::default()
        }))
    }
}

impl Into<Option<AuthTicketEncoded>> for RenewAuthTicketResponsePb {
    fn into(self) -> Option<AuthTicketEncoded> {
        match (self.user, self.token) {
            (Some(user), Some(token)) => {
                let user: AuthUserExtract = user.into();
                let token: Option<AuthTokenEncoded> = token.into();
                token.map(|token| AuthTicketEncoded { user, token })
            }
            _ => None,
        }
    }
}
