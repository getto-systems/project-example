use crate::x_outside_feature::{auth::feature::AuthAppFeature, data::RequestId};

use crate::auth::{
    ticket::authorize::init::ActiveAuthorizeInfra,
    user::kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
};

use crate::auth::user::password::reset::token_destination::change::action::{
    ChangeResetTokenDestinationAction, ChangeResetTokenDestinationMaterial,
};

pub struct ActiveChangeResetTokenDestinationMaterial<'a> {
    authorize: ActiveAuthorizeInfra<'a>,
    destination_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> ActiveChangeResetTokenDestinationMaterial<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        request_id: RequestId,
    ) -> ChangeResetTokenDestinationAction<Self> {
        ChangeResetTokenDestinationAction::with_material(Self {
            authorize: ActiveAuthorizeInfra::from_auth(feature, request_id),
            destination_repository: DynamoDbAuthUserRepository::new(&feature.store),
        })
    }
}

impl<'a> ChangeResetTokenDestinationMaterial for ActiveChangeResetTokenDestinationMaterial<'a> {
    type Authorize = ActiveAuthorizeInfra<'a>;

    type DestinationRepository = DynamoDbAuthUserRepository<'a>;

    fn authorize(&self) -> &Self::Authorize {
        &self.authorize
    }
    fn destination_repository(&self) -> &Self::DestinationRepository {
        &self.destination_repository
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::{
        ticket::authorize::init::test::StaticAuthorizeInfra,
        user::kernel::init::user_repository::memory::MemoryAuthUserRepository,
    };

    use crate::auth::user::password::reset::token_destination::change::action::ChangeResetTokenDestinationMaterial;

    use crate::auth::user::password::reset::token_destination::change::infra::{
        ChangeResetTokenDestinationFields, ChangeResetTokenDestinationFieldsExtract,
    };

    use crate::auth::user::password::reset::token_destination::change::data::ValidateChangeResetTokenDestinationFieldsError;

    pub enum StaticChangeResetTokenDestinationFields {
        Valid(ChangeResetTokenDestinationFields),
        Invalid(ValidateChangeResetTokenDestinationFieldsError),
    }

    impl ChangeResetTokenDestinationFieldsExtract for StaticChangeResetTokenDestinationFields {
        fn convert(
            self,
        ) -> Result<ChangeResetTokenDestinationFields, ValidateChangeResetTokenDestinationFieldsError>
        {
            match self {
                Self::Valid(fields) => Ok(fields),
                Self::Invalid(err) => Err(err),
            }
        }
    }

    pub struct StaticChangeResetTokenDestinationMaterial<'a> {
        pub authorize: StaticAuthorizeInfra,
        pub destination_repository: MemoryAuthUserRepository<'a>,
    }

    impl<'a> ChangeResetTokenDestinationMaterial for StaticChangeResetTokenDestinationMaterial<'a> {
        type Authorize = StaticAuthorizeInfra;

        type DestinationRepository = MemoryAuthUserRepository<'a>;

        fn authorize(&self) -> &Self::Authorize {
            &self.authorize
        }
        fn destination_repository(&self) -> &Self::DestinationRepository {
            &self.destination_repository
        }
    }
}
