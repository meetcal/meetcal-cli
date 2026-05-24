use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiftingResults {
    pub id: Option<u64>,
    pub convex_id: Option<String>,
    pub event_id: String,
    pub federation: String,
    pub legacy_id: Option<f64>,
    pub meet: String,
    pub date: String,
    pub name: String,
    pub age: String,
    pub body_weight: f64,
    pub snatch1: f64,
    pub snatch2: f64,
    pub snatch3: f64,
    pub snatch_best: f64,
    pub cj1: f64,
    pub cj2: f64,
    pub cj3: f64,
    pub cj_best: f64,
    pub total: f64,
    pub adaptive: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveRecord {
    pub weight_class: String,
    pub snatch: f64,
    pub cj: f64,
    pub total: f64,
}

pub struct PRs {
    pub snatch_best: f64,
    pub cj_best: f64,
    pub total_best: f64,
}
