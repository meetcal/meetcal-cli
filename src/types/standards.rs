use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Standards {
    pub weight_class: String,
    pub standard_a: i32,
    pub standard_b: i32,
}
