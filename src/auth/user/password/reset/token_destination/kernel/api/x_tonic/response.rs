use crate::auth::user::password::reset::token_destination::kernel::y_protobuf::service::ResetTokenDestinationPb;

use crate::auth::user::password::reset::kernel::data::ResetTokenDestination;

impl Into<ResetTokenDestinationPb> for ResetTokenDestination {
    fn into(self) -> ResetTokenDestinationPb {
        match self {
            ResetTokenDestination::None => ResetTokenDestinationPb {
                r#type: "none".into(),
                ..Default::default()
            },
            ResetTokenDestination::Email(email) => ResetTokenDestinationPb {
                r#type: "email".into(),
                email: email.extract(),
            },
        }
    }
}
