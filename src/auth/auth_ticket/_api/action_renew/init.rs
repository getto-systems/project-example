use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use super::super::encode::init::RenewEncodeAuthTicketStruct;
use super::super::validate::init::TicketValidateAuthTokenStruct;

use super::action::RenewAuthTicketAction;
use super::action::RenewAuthTicketMaterial;

impl<'a> RenewAuthTicketAction<RenewAuthTicketFeature<'a>> {
    pub fn new(request: &'a HttpRequest, feature: &'a AuthOutsideFeature) -> Self {
        Self::with_material(RenewAuthTicketFeature::new(request, feature))
    }
}

pub struct RenewAuthTicketFeature<'a> {
    validate: TicketValidateAuthTokenStruct<'a>,
    encode: RenewEncodeAuthTicketStruct<'a>,
}

impl<'a> RenewAuthTicketFeature<'a> {
    fn new(request: &'a HttpRequest, feature: &'a AuthOutsideFeature) -> Self {
        Self {
            validate: TicketValidateAuthTokenStruct::new(request, feature),
            encode: RenewEncodeAuthTicketStruct::new(feature),
        }
    }
}

impl<'a> RenewAuthTicketMaterial for RenewAuthTicketFeature<'a> {
    type Validate = TicketValidateAuthTokenStruct<'a>;
    type Encode = RenewEncodeAuthTicketStruct<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }
}
