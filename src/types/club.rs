use super::lifting_results::LiftingResults;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AthleteClub {
    pub member_id: String,
    pub name: String,
    pub club: String,
    pub meet: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClubMeetStats {
    pub total_athletes: u32,
    pub gold_medals: u32,
    pub silver_medals: u32,
    pub bronze_medals: u32,
    pub total_prs: u32,
    pub perfect6for6: u32,
    pub total_weight_lifted: u32,
    pub athlete_results: Vec<LiftingResults>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AthleteInfo {
    pub name: String,
    pub age: u32,
    pub gender: String,
    pub weight_class: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AthleteWeightClass {
    pub name: String,
    pub weight_class: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeetStatus {
    pub name: String,
    pub status: String,
}
