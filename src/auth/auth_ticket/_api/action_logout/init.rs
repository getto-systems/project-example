use actix_web::HttpRequest;

use crate::x_outside_feature::_api::feature::AppFeature;

use crate::auth::auth_ticket::_api::logout::init::LogoutStruct;

use super::action::{LogoutAction, LogoutMaterial};

pub struct LogoutFeature<'a> {
    logout: LogoutStruct<'a>,
}

impl<'a> LogoutFeature<'a> {
    pub fn action(
        feature: &'a AppFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> LogoutAction<Self> {
        LogoutAction::with_material(Self {
            logout: LogoutStruct::new(&feature.auth, request_id, request),
        })
    }
}

impl<'a> LogoutMaterial for LogoutFeature<'a> {
    type Logout = LogoutStruct<'a>;

    fn logout(&self) -> &Self::Logout {
        &self.logout
    }
}
