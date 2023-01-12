use crate::auth::x_outside_feature::feature::AuthOutsideCloudfrontKey;

use crate::auth::ticket::encode::infra::CdnTokenEncoder;

use crate::auth::{
    kernel::data::ExpireDateTime,
    ticket::{
        encode::data::EncodeAuthTokenError,
        kernel::data::{AWSCloudfrontToken, CdnToken},
    },
};

pub struct AWSCloudfrontCdnTokenEncoder<'a> {
    secret: &'a AuthOutsideCloudfrontKey,
}

impl<'a> AWSCloudfrontCdnTokenEncoder<'a> {
    pub fn new(secret: &'a AuthOutsideCloudfrontKey) -> Self {
        Self { secret }
    }
}

impl<'a> CdnTokenEncoder for AWSCloudfrontCdnTokenEncoder<'a> {
    fn encode(
        &self,
        expires: ExpireDateTime,
    ) -> Result<(CdnToken, ExpireDateTime), EncodeAuthTokenError> {
        let policy = aws_cloudfront_cookie::CloudfrontPolicy::from_resource(
            self.secret.resource.into(),
            expires.clone().extract_to_timestamp(),
        );
        let content = self
            .secret
            .key
            .sign(policy)
            .map_err(|err| EncodeAuthTokenError::InfraError(format!("sign error: {}", err)))?;

        Ok((
            CdnToken::AWSCloudfront(AWSCloudfrontToken {
                key_pair_id: self.secret.key_pair_id.into(),
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
            encode::data::EncodeAuthTokenError,
            kernel::data::{AWSCloudfrontToken, CdnToken},
        },
    };

    pub struct StaticCdnTokenEncoder;

    impl CdnTokenEncoder for StaticCdnTokenEncoder {
        fn encode(
            &self,
            expires: ExpireDateTime,
        ) -> Result<(CdnToken, ExpireDateTime), EncodeAuthTokenError> {
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
