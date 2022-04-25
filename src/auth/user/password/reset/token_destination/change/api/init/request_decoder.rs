use crate::auth::user::password::reset::token_destination::{
    change::y_protobuf::service::ChangeResetTokenDestinationRequestPb,
    kernel::y_protobuf::service::ResetTokenDestinationPb,
};

use crate::auth::user::password::reset::token_destination::change::infra::{
    ChangeResetTokenDestinationFieldsExtract, ChangeResetTokenDestinationRequestDecoder,
};

use crate::auth::user::password::reset::kernel::data::ResetTokenDestinationExtract;

pub struct PbChangeResetTokenDestinationRequestDecoder {
    request: ChangeResetTokenDestinationRequestPb,
}

impl PbChangeResetTokenDestinationRequestDecoder {
    pub const fn new(request: ChangeResetTokenDestinationRequestPb) -> Self {
        Self { request }
    }
}

impl ChangeResetTokenDestinationRequestDecoder for PbChangeResetTokenDestinationRequestDecoder {
    fn decode(self) -> ChangeResetTokenDestinationFieldsExtract {
        ChangeResetTokenDestinationFieldsExtract {
            login_id: self.request.login_id,
            from: self.request.from.map(decode_destination),
            to: self.request.to.map(decode_destination),
        }
    }
}

fn decode_destination(request: ResetTokenDestinationPb) -> ResetTokenDestinationExtract {
    match request.r#type.as_str() {
        "email" => ResetTokenDestinationExtract::Email(request.email),
        _ => ResetTokenDestinationExtract::None,
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::reset::token_destination::change::infra::{
        ChangeResetTokenDestinationFieldsExtract, ChangeResetTokenDestinationRequestDecoder,
    };

    pub enum StaticChangeResetTokenDestinationRequestDecoder {
        Valid(ChangeResetTokenDestinationFieldsExtract),
    }

    impl ChangeResetTokenDestinationRequestDecoder for StaticChangeResetTokenDestinationRequestDecoder {
        fn decode(self) -> ChangeResetTokenDestinationFieldsExtract {
            match self {
                Self::Valid(fields) => fields,
            }
        }
    }
}
