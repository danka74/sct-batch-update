use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BatchSettings {
    pub name: String,
    pub batch_type: String,
    pub inactivation_reason: String,
    pub ecl: String,
    pub term: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Criteria {
    pub qualifier: String,
    pub present: bool,
    pub lang: String,
    pub desc_type: String,
    pub accept: String,
    pub regexp: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Replace
 {
    pub lang: String,
    pub replace: String,
    pub replace_with: String
 }

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BatchChangeRequest {
    pub settings: BatchSettings,
    pub criteria: Vec<Criteria>,
    pub replace: Vec<Replace>
}

#[derive(Debug, Default, Serialize)]
pub struct BatchResponse {
    pub status: String
}