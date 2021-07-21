use tonic::Request;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_auth::{
    discard::init::DiscardAuthTicketStruct, validate::init::TicketValidateAuthTokenStruct,
};

use super::action::{LogoutAction, LogoutMaterial};

impl<'a> LogoutAction<LogoutFeature<'a>> {
    pub fn new<T>(feature: &'a AuthOutsideFeature, request: &'a Request<T>) -> Self {
        Self::with_material(LogoutFeature::new(feature, request))
    }
}

pub struct LogoutFeature<'a> {
    validate: TicketValidateAuthTokenStruct<'a>,
    discard: DiscardAuthTicketStruct<'a>,
}

impl<'a> LogoutFeature<'a> {
    fn new<T>(feature: &'a AuthOutsideFeature, request: &'a Request<T>) -> Self {
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
