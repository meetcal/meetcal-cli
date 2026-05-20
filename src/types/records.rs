use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Gender {
    Men,
    Women,
}

#[derive(Debug, Deserialize)]
pub enum Federation {
    Iwf,
    Usamw,
    Usaw,
    Umwf,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub weight_class: String,
    pub snatch_record: i32,
    pub cj_record: i32,
    pub total_record: i32,
    pub gender: Gender,
    pub age_category: String,
    pub record_type: Federation,
}
