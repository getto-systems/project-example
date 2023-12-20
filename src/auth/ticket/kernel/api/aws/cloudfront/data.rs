use rsa::RsaPrivateKey;
use serde::Serialize;

pub struct AwsCloudfrontKey {
    pub private_key: RsaPrivateKey,
}

pub struct AwsCloudfrontSignedContent {
    pub policy: String,
    pub signature: String,
}

#[derive(Serialize)]
pub struct AwsCloudfrontPolicy {
    #[serde(rename = "Statement")]
    pub statement: Vec<AwsCloudfrontStatement>,
}

#[derive(Serialize)]
pub struct AwsCloudfrontStatement {
    #[serde(rename = "Resource")]
    pub resource: String,
    #[serde(rename = "Condition")]
    pub condition: AwsCloudfrontCondition,
}

#[derive(Serialize)]
pub struct AwsCloudfrontCondition {
    #[serde(rename = "DateLessThan")]
    pub date_less_than: AwsCloudfrontConditionDateLessThan,
}

#[derive(Serialize)]
pub struct AwsCloudfrontConditionDateLessThan {
    #[serde(rename = "AWS:EpochTime")]
    pub epoch_time: i64,
}
