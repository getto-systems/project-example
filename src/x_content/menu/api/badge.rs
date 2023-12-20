use crate::{
    common::api::repository::data::RepositoryError,
    common::outline::load::data::OutlineMenuBadgeCount,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutlineMenuBadgePath {
    Index,
}

impl OutlineMenuBadgePath {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Index => "index",
        }
    }

    pub fn extract(self) -> String {
        self.as_str().to_owned()
    }
}

pub struct GatherOutlineMenuBadgeAction {
    example: GatherExampleMenuBadgeAction,
}

impl GatherOutlineMenuBadgeAction {
    pub async fn gather(
        &self,
    ) -> Result<Vec<(OutlineMenuBadgePath, OutlineMenuBadgeCount)>, RepositoryError> {
        Ok(vec![(
            OutlineMenuBadgePath::Index,
            self.example.menu_badge_count().await?,
        )])
    }
}

struct GatherExampleMenuBadgeAction;

impl GatherExampleMenuBadgeAction {
    async fn menu_badge_count(&self) -> Result<OutlineMenuBadgeCount, RepositoryError> {
        Ok(OutlineMenuBadgeCount::restore(0))
    }
}

mod x_tonic {
    use std::sync::Arc;

    use crate::x_outside_feature::core::feature::CoreAppFeature;

    use crate::common::api::{logger::detail::StdoutJsonLogger, request::data::RequestInfo};

    impl super::GatherOutlineMenuBadgeAction {
        pub fn live(
            _feature: &Arc<CoreAppFeature>,
            _info: RequestInfo,
            _logger: &Arc<StdoutJsonLogger>,
        ) -> Self {
            Self {
                example: super::GatherExampleMenuBadgeAction,
            }
        }
    }
}

mod test {
    impl super::GatherOutlineMenuBadgeAction {
        pub fn mock() -> Self {
            Self {
                example: super::GatherExampleMenuBadgeAction,
            }
        }
    }
}
