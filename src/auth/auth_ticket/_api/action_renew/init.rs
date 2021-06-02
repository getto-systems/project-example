use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use super::super::encode::init::EncodeRenewAuthTicketStruct;
use super::super::validate::init::ValidateTicketTokenStruct;

use super::action::RenewAuthTicketAction;
use super::action::RenewAuthTicketMaterial;

impl<'a> RenewAuthTicketAction<RenewAuthTicketFeature<'a>> {
    pub fn new(request: HttpRequest, feature: &'a AuthOutsideFeature) -> Self {
        Self::with_material(RenewAuthTicketFeature::new(request, feature))
    }
}

pub struct RenewAuthTicketFeature<'a> {
    validate: ValidateTicketTokenStruct<'a>,
    encode: EncodeRenewAuthTicketStruct<'a>,
}

impl<'a> RenewAuthTicketFeature<'a> {
    fn new(request: HttpRequest, feature: &'a AuthOutsideFeature) -> Self {
        Self {
            validate: ValidateTicketTokenStruct::new(request, feature),
            encode: EncodeRenewAuthTicketStruct::new(feature),
        }
    }
}

impl<'a> RenewAuthTicketMaterial for RenewAuthTicketFeature<'a> {
    type Validate = ValidateTicketTokenStruct<'a>;
    type Encode = EncodeRenewAuthTicketStruct<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }
}
