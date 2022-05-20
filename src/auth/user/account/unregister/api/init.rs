pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::user::account::unregister::y_protobuf::service::UnregisterAuthUserAccountRequestPb;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::{
        kernel::init::ticket_repository::dynamodb::DynamoDbAuthTicketRepository,
        validate::init::AuthenticateApiStruct,
    },
    user::{
        account::unregister::init::request_decoder::PbUnregisterAuthUserAccountRequestDecoder,
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
    },
};

use super::action::{UnregisterAuthUserAccountAction, UnregisterAuthUserAccountMaterial};

pub struct UnregisterAuthUserAccountFeature<'a> {
    validate: AuthenticateApiStruct<'a>,
    ticket_repository: DynamoDbAuthTicketRepository<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> UnregisterAuthUserAccountFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: UnregisterAuthUserAccountRequestPb,
    ) -> UnregisterAuthUserAccountAction<PbUnregisterAuthUserAccountRequestDecoder, Self> {
        UnregisterAuthUserAccountAction::with_material(
            PbUnregisterAuthUserAccountRequestDecoder::new(request),
            Self {
                validate: AuthenticateApiStruct::new(feature, metadata),
                ticket_repository: DynamoDbAuthTicketRepository::new(&feature.store),
                user_repository: DynamoDbAuthUserRepository::new(&feature.store),
            },
        )
    }
}

impl<'a> UnregisterAuthUserAccountMaterial for UnregisterAuthUserAccountFeature<'a> {
    type Authenticate = AuthenticateApiStruct<'a>;

    type TicketRepository = DynamoDbAuthTicketRepository<'a>;
    type UserRepository = DynamoDbAuthUserRepository<'a>;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.validate
    }

    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}
