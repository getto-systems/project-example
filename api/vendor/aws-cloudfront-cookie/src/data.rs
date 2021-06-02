use serde::Serialize;

pub struct SignedContent {
    pub policy: String,
    pub signature: String,
}

#[derive(Serialize)]
pub struct Policy {
    #[serde(rename = "Statement")]
    pub statement: Vec<Statement>,
}
#[derive(Serialize)]
pub struct Statement {
    #[serde(rename = "Resource")]
    pub resource: String,
    #[serde(rename = "Condition")]
    pub condition: Condition,
}
#[derive(Serialize)]
pub struct Condition {
    #[serde(rename = "DateLessThan")]
    pub date_less_than: ConditionDateLessThan,
}
#[derive(Serialize)]
pub struct ConditionDateLessThan {
    #[serde(rename = "AWS:EpochTime")]
    pub epoch_time: i64,
}
