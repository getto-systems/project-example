pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::user::password::reset::token_destination::change::y_protobuf::service::ChangeResetTokenDestinationRequestPb;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::validate::init::AuthenticateApiStruct,
    user::{
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
        password::reset::token_destination::change::init::request_decoder::PbChangeResetTokenDestinationRequestDecoder,
    },
};

use super::action::{ChangeResetTokenDestinationAction, ChangeResetTokenDestinationMaterial};

pub struct ChangeResetTokenDestinationFeature<'a> {
    validate: AuthenticateApiStruct<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> ChangeResetTokenDestinationFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: ChangeResetTokenDestinationRequestPb,
    ) -> ChangeResetTokenDestinationAction<PbChangeResetTokenDestinationRequestDecoder, Self> {
        ChangeResetTokenDestinationAction::with_material(
            PbChangeResetTokenDestinationRequestDecoder::new(request),
            Self {
                validate: AuthenticateApiStruct::new(feature, metadata),
                user_repository: DynamoDbAuthUserRepository::new(&feature.store),
            },
        )
    }
}

impl<'a> ChangeResetTokenDestinationMaterial for ChangeResetTokenDestinationFeature<'a> {
    type Authenticate = AuthenticateApiStruct<'a>;

    type DestinationRepository = DynamoDbAuthUserRepository<'a>;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.validate
    }
    fn destination_repository(&self) -> &Self::DestinationRepository {
        &self.user_repository
    }
}
