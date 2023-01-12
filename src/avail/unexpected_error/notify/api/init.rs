use crate::x_outside_feature::{core::feature::CoreAppFeature, data::RequestId};

use crate::auth::init::ActiveAuthorizeInfra;

use crate::avail::unexpected_error::notify::action::{
    NotifyUnexpectedErrorAction, NotifyUnexpectedErrorMaterial,
};

pub struct ActiveNotifyUnexpectedErrorMaterial<'a> {
    authorize: ActiveAuthorizeInfra<'a>,
}

impl<'a> ActiveNotifyUnexpectedErrorMaterial<'a> {
    pub fn action(
        feature: &'a CoreAppFeature,
        request_id: RequestId,
    ) -> NotifyUnexpectedErrorAction<Self> {
        NotifyUnexpectedErrorAction::with_material(Self {
            authorize: ActiveAuthorizeInfra::from_service(&feature.auth, request_id),
        })
    }
}

#[async_trait::async_trait]
impl<'a> NotifyUnexpectedErrorMaterial for ActiveNotifyUnexpectedErrorMaterial<'a> {
    type Authorize = ActiveAuthorizeInfra<'a>;

    fn authorize(&self) -> &Self::Authorize {
        &self.authorize
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::init::test::StaticAuthorizeInfra;

    use crate::avail::unexpected_error::notify::action::NotifyUnexpectedErrorMaterial;

    use crate::avail::unexpected_error::notify::infra::{
        NotifyUnexpectedErrorFields, NotifyUnexpectedErrorFieldsExtract,
    };

    pub struct StaticNotifyUnexpectedErrorFields {
        pub fields: NotifyUnexpectedErrorFields,
    }

    impl NotifyUnexpectedErrorFieldsExtract for StaticNotifyUnexpectedErrorFields {
        fn convert(self) -> NotifyUnexpectedErrorFields {
            self.fields
        }
    }

    pub struct StaticNotifyUnexpectedErrorMaterial {
        pub authorize: StaticAuthorizeInfra,
    }

    #[async_trait::async_trait]
    impl NotifyUnexpectedErrorMaterial for StaticNotifyUnexpectedErrorMaterial {
        type Authorize = StaticAuthorizeInfra;

        fn authorize(&self) -> &Self::Authorize {
            &self.authorize
        }
    }
}
