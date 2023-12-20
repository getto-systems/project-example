use std::sync::Arc;

use crate::{
    auth::x_outside_feature::feature::AuthOutsideCloudfrontKey,
    x_outside_feature::auth::feature::AuthAppFeature,
};

use crate::common::api::feature::AsInfra;

use crate::auth::ticket::encode::infra::CdnTokenEncoder;

use crate::auth::{
    kernel::data::ExpireDateTime,
    ticket::{
        encode::data::EncodeTokenError,
        kernel::{
            aws::cloudfront::data::AwsCloudfrontPolicy,
            data::{AWSCloudfrontToken, CdnToken},
        },
    },
};

pub struct AWSCloudfrontCdnTokenEncoder {
    key: Arc<AuthOutsideCloudfrontKey>,
}

impl AsInfra<AWSCloudfrontCdnTokenEncoder> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> AWSCloudfrontCdnTokenEncoder {
        AWSCloudfrontCdnTokenEncoder {
            key: Arc::clone(&self.cloudfront_key),
        }
    }
}

impl CdnTokenEncoder for AWSCloudfrontCdnTokenEncoder {
    fn encode(
        &self,
        expires: ExpireDateTime,
    ) -> Result<(CdnToken, ExpireDateTime), EncodeTokenError> {
        let policy = AwsCloudfrontPolicy::from_resource(
            self.key.resource.into(),
            expires.clone().extract_to_timestamp(),
        );
        let content = self
            .key
            .key
            .sign(policy)
            .map_err(|err| EncodeTokenError::InfraError(format!("sign error: {}", err)))?;

        Ok((
            CdnToken::AWSCloudfront(AWSCloudfrontToken {
                key_pair_id: self.key.key_pair_id.into(),
                policy: content.policy,
                signature: content.signature,
            }),
            expires,
        ))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::encode::infra::CdnTokenEncoder;

    use crate::auth::{
        kernel::data::ExpireDateTime,
        ticket::{
            encode::data::EncodeTokenError,
            kernel::data::{AWSCloudfrontToken, CdnToken},
        },
    };

    pub struct StaticCdnTokenEncoder;

    impl CdnTokenEncoder for StaticCdnTokenEncoder {
        fn encode(
            &self,
            expires: ExpireDateTime,
        ) -> Result<(CdnToken, ExpireDateTime), EncodeTokenError> {
            Ok((
                CdnToken::AWSCloudfront(AWSCloudfrontToken {
                    key_pair_id: "KEY-PAIR-ID".to_owned(),
                    policy: "POLICY".to_owned(),
                    signature: "SIGNATURE".to_owned(),
                }),
                expires,
            ))
        }
    }
}
