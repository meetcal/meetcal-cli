#[derive(Debug)]
pub enum Movement {
    Snatch,
    CleanAndJerk,
    Total,
}

#[derive(Debug)]
pub struct AthleteRow {
    pub name: String,
    pub wso: Option<String>,
}

#[derive(Debug)]
pub struct ClubPrDetail {
    pub name: String,
    pub movement: Movement,
    pub new_pr: i32,
    pub previous_pr: i32,
}

#[derive(Debug)]
pub struct ClubMedalDetail {
    pub name: String,
    pub age: String,
    pub movement: Movement,
    pub place: u32,
    pub result: i32,
}
