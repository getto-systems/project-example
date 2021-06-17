use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use super::super::discard::init::DiscardAuthTicketStruct;
use super::super::validate::init::TicketValidateAuthTokenStruct;

use super::action::LogoutAction;
use super::action::LogoutMaterial;

impl<'a> LogoutAction<LogoutFeature<'a>> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest) -> Self {
        Self::with_material(LogoutFeature::new(feature, request))
    }
}

pub struct LogoutFeature<'a> {
    validate: TicketValidateAuthTokenStruct<'a>,
    discard: DiscardAuthTicketStruct<'a>,
}

impl<'a> LogoutFeature<'a> {
    fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest) -> Self {
        Self {
            validate: TicketValidateAuthTokenStruct::new(feature, request),
            discard: DiscardAuthTicketStruct::new(feature),
        }
    }
}

impl<'a> LogoutMaterial for LogoutFeature<'a> {
    type Validate = TicketValidateAuthTokenStruct<'a>;
    type Discard = DiscardAuthTicketStruct<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn discard(&self) -> &Self::Discard {
        &self.discard
    }
}
