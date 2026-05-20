use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiftingResults {
    pub id: Option<u64>,
    pub convex_id: Option<String>,
    pub event_id: String,
    pub federation: Option<String>,
    pub legacy_id: Option<u64>,
    pub meet: String,
    pub date: String,
    pub name: String,
    pub age: String,
    pub body_weight: f64,
    pub weight_class: Option<String>,
    pub snatch1: i32,
    pub snatch2: i32,
    pub snatch3: i32,
    pub snatch_best: i32,
    pub cj1: i32,
    pub cj2: i32,
    pub cj3: i32,
    pub cj_best: i32,
    pub total: i32,
    pub adaptive: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveRecord {
    pub weight_class: String,
    pub snatch: i32,
    pub cj: i32,
    pub total: i32,
}
