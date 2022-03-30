use tonic::{Response, Status};

use crate::auth::user::account::modify::y_protobuf::service::{
    ModifyAuthUserAccountDataPb, ModifyAuthUserAccountErrorKindPb, ModifyAuthUserAccountResponsePb,
    ModifyResetTokenDestinationDataPb,
};

use crate::auth::user::password::reset::kernel::data::ResetTokenDestinationExtract;
use crate::z_lib::response::tonic::ServiceResponder;

use super::super::action::{ModifyAuthUserAccountEvent, ModifyAuthUserAccountState};

impl ServiceResponder<ModifyAuthUserAccountResponsePb> for ModifyAuthUserAccountState {
    fn respond_to(self) -> Result<Response<ModifyAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::ModifyUser(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<ModifyAuthUserAccountResponsePb> for ModifyAuthUserAccountEvent {
    fn respond_to(self) -> Result<Response<ModifyAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Success(user) => Ok(Response::new(ModifyAuthUserAccountResponsePb {
                success: true,
                data: Some(ModifyAuthUserAccountDataPb {
                    granted_roles: user.granted_roles.extract().into_iter().collect(),
                    reset_token_destination: match user.reset_token_destination.extract() {
                        ResetTokenDestinationExtract::None => {
                            Some(ModifyResetTokenDestinationDataPb {
                                r#type: "none".into(),
                                ..Default::default()
                            })
                        }
                        ResetTokenDestinationExtract::Email(email) => {
                            Some(ModifyResetTokenDestinationDataPb {
                                r#type: "email".into(),
                                email,
                            })
                        }
                    },
                }),
                ..Default::default()
            })),
            Self::UserNotFound => Ok(Response::new(ModifyAuthUserAccountResponsePb {
                success: false,
                // ユーザーが見つからなかった場合も invalid login id エラーを返す
                err: ModifyAuthUserAccountErrorKindPb::InvalidLoginId as i32,
                ..Default::default()
            })),
            Self::Conflict => Ok(Response::new(ModifyAuthUserAccountResponsePb {
                success: false,
                err: ModifyAuthUserAccountErrorKindPb::Conflict as i32,
                ..Default::default()
            })),
            Self::InvalidUser(_) => Ok(Response::new(ModifyAuthUserAccountResponsePb {
                success: false,
                err: ModifyAuthUserAccountErrorKindPb::InvalidUser as i32,
                ..Default::default()
            })),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
