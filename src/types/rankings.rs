use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rankings {
    pub ranking: f64,
    pub name: String,
    pub weight_class: String,
    pub percent_a: f64,
    pub total: f64,
}
