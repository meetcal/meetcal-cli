use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum CarolinaTab {
    Youth,
    Junior,
    Senior,
    Masters,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsoOwlCmsReferenceMeta {
    pub age_min: String,
    pub age_max: String,
    pub body_weight_min: String,
    pub body_weight_max: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsoOwlCmsParsedLift {
    pub lift: String,
    pub record: Option<i32>,
    pub name: String,
    pub date: String,
    pub place: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsoOwlCmsParsedBlock {
    pub weight_class: String,
    pub age_group: String,
    pub gender_code: String,
    pub lifts: Vec<WsoOwlCmsParsedLift>,
}
