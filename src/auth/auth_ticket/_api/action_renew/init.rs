use actix_web::HttpRequest;

use crate::x_outside_feature::_api::feature::AppFeature;

use crate::auth::auth_ticket::_api::renew::init::RenewAuthTicketStruct;

use super::action::{RenewAuthTicketAction, RenewAuthTicketMaterial};

pub struct RenewAuthTicketFeature<'a> {
    renew: RenewAuthTicketStruct<'a>,
}

impl<'a> RenewAuthTicketFeature<'a> {
    pub fn action(
        feature: &'a AppFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> RenewAuthTicketAction<Self> {
        RenewAuthTicketAction::with_material(Self {
            renew: RenewAuthTicketStruct::new(&feature.auth, request_id, request),
        })
    }
}

impl<'a> RenewAuthTicketMaterial for RenewAuthTicketFeature<'a> {
    type Renew = RenewAuthTicketStruct<'a>;

    fn renew(&self) -> &Self::Renew {
        &self.renew
    }
}
