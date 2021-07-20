use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::renew::init::RenewAuthTicketStruct;

use super::action::{RenewAuthTicketAction, RenewAuthTicketMaterial};

impl<'a> RenewAuthTicketAction<RenewAuthTicketFeature<'a>> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self::with_material(RenewAuthTicketFeature::new(feature, request_id, request))
    }
}

pub struct RenewAuthTicketFeature<'a> {
    renew: RenewAuthTicketStruct<'a>,
}

impl<'a> RenewAuthTicketFeature<'a> {
    fn new(feature: &'a AuthOutsideFeature, request_id: &'a str, request: &'a HttpRequest) -> Self {
        Self {
            renew: RenewAuthTicketStruct::new(feature, request_id, request),
        }
    }
}

impl<'a> RenewAuthTicketMaterial for RenewAuthTicketFeature<'a> {
    type Renew = RenewAuthTicketStruct<'a>;

    fn renew(&self) -> &Self::Renew {
        &self.renew
    }
}
