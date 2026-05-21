use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Standards {
    pub weight_class: String,
    pub standard_a: f64,
    pub standard_b: f64,
}
