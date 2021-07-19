use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::logout::init::LogoutStruct;

use super::action::{LogoutAction, LogoutMaterial};

impl<'a> LogoutAction<LogoutFeature<'a>> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self::with_material(LogoutFeature::new(feature, request_id, request))
    }
}

pub struct LogoutFeature<'a> {
    logout: LogoutStruct<'a>,
}

impl<'a> LogoutFeature<'a> {
    fn new(feature: &'a AuthOutsideFeature, request_id: &'a str, request: &'a HttpRequest) -> Self {
        Self {
            logout: LogoutStruct::new(feature, request_id, request),
        }
    }
}

impl<'a> LogoutMaterial for LogoutFeature<'a> {
    type Logout = LogoutStruct<'a>;

    fn logout(&self) -> &Self::Logout {
        &self.logout
    }
}
