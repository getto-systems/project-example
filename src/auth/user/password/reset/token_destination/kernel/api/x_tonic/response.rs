use crate::auth::user::password::reset::kernel::y_protobuf::service::ResetTokenDestinationPb;

use crate::auth::user::password::reset::kernel::data::ResetPasswordTokenDestination;

impl Into<ResetTokenDestinationPb> for ResetPasswordTokenDestination {
    fn into(self) -> ResetTokenDestinationPb {
        match self {
            ResetPasswordTokenDestination::None => ResetTokenDestinationPb {
                r#type: "none".into(),
                ..Default::default()
            },
            ResetPasswordTokenDestination::Email(email) => ResetTokenDestinationPb {
                r#type: "email".into(),
                email: email.extract(),
            },
        }
    }
}
