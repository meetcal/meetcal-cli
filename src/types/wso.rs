use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Movement {
    Snatch,
    CleanAndJerk,
    Total,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSORecord {
    pub weight_class: String,
    pub snatch_record: f64,
    pub cj_record: f64,
    pub total_record: f64,
    pub gender: String,
    pub age_category: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AthleteRow {
    pub name: String,
    pub wso: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClubPrDetail {
    pub name: String,
    pub movement: Movement,
    pub new_pr: i32,
    pub previous_pr: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClubMedalDetail {
    pub name: String,
    pub age: String,
    pub movement: Movement,
    pub place: u32,
    pub result: i32,
}
