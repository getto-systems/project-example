use tonic::Request;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_auth::{
    discard::init::DiscardAuthTicketStruct, validate::init::TicketValidateAuthTokenStruct,
};

use super::action::{LogoutAction, LogoutMaterial};

impl<'a, T> LogoutAction<LogoutFeature<'a, T>> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a Request<T>) -> Self {
        Self::with_material(LogoutFeature::new(feature, request))
    }
}

pub struct LogoutFeature<'a, T> {
    validate: TicketValidateAuthTokenStruct<'a, T>,
    discard: DiscardAuthTicketStruct<'a>,
}

impl<'a, T> LogoutFeature<'a, T> {
    fn new(feature: &'a AuthOutsideFeature, request: &'a Request<T>) -> Self {
        Self {
            validate: TicketValidateAuthTokenStruct::new(feature, request),
            discard: DiscardAuthTicketStruct::new(feature),
        }
    }
}

impl<'a, T> LogoutMaterial for LogoutFeature<'a, T> {
    type Validate = TicketValidateAuthTokenStruct<'a, T>;
    type Discard = DiscardAuthTicketStruct<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn discard(&self) -> &Self::Discard {
        &self.discard
    }
}
