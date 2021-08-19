use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::avail::_api::notify_unexpected_error::{
    infra::NotifyUnexpectedErrorRequestDecoder,
    init::request_decoder::ProstNotifyUnexpectedErrorRequestDecoder,
};

use crate::avail::_api::notify_unexpected_error::init::NotifyUnexpectedErrorStruct;

use super::action::{NotifyUnexpectedErrorAction, NotifyUnexpectedErrorMaterial};

pub struct NotifyUnexpectedErrorFeature<'a> {
    notify: NotifyUnexpectedErrorStruct<'a>,
}

impl<'a> NotifyUnexpectedErrorFeature<'a> {
    pub fn action(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> NotifyUnexpectedErrorAction<Self> {
        NotifyUnexpectedErrorAction::with_material(Self {
            notify: NotifyUnexpectedErrorStruct::new(feature, request_id, request),
        })
    }
    pub fn request_decoder(body: String) -> impl NotifyUnexpectedErrorRequestDecoder {
        ProstNotifyUnexpectedErrorRequestDecoder::new(body)
    }
}

impl<'a> NotifyUnexpectedErrorMaterial for NotifyUnexpectedErrorFeature<'a> {
    type Notify = NotifyUnexpectedErrorStruct<'a>;

    fn notify(&self) -> &Self::Notify {
        &self.notify
    }
}
