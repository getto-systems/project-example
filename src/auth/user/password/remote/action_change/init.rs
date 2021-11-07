use tonic::metadata::MetadataMap;

use crate::auth::user::password::remote::y_protobuf::service::ChangePasswordRequestPb;

use crate::x_outside_feature::remote::auth::feature::AuthAppFeature;

use crate::auth::user::password::remote::change::init::{
    request_decoder::PbChangePasswordRequestDecoder, ChangePasswordStruct,
};

use super::action::{ChangePasswordAction, ChangePasswordMaterial};

use crate::auth::user::password::remote::change::infra::ChangePasswordRequestDecoder;

pub struct ChangePasswordFeature<'a> {
    change: ChangePasswordStruct<'a>,
}

impl<'a> ChangePasswordFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
    ) -> ChangePasswordAction<Self> {
        ChangePasswordAction::with_material(Self {
            change: ChangePasswordStruct::new(&feature.auth, metadata),
        })
    }
    pub fn request_decoder(request: ChangePasswordRequestPb) -> impl ChangePasswordRequestDecoder {
        PbChangePasswordRequestDecoder::new(request)
    }
}

impl<'a> ChangePasswordMaterial for ChangePasswordFeature<'a> {
    type Change = ChangePasswordStruct<'a>;

    fn change(&self) -> &Self::Change {
        &self.change
    }
}
