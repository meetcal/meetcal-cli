use clap::Parser;

/// Search for WSO Records for a given age, wso, and gender.
///
/// Examples:
///   meetcal wsoRecords --age Senior --gender Men --wso Carolinas
///   meetcal wsoRecords U17 Women Carolinas
#[derive(Parser)]
#[command(name = "wsoRecords")]
pub struct WsoRecordsArgs {
    /// Age group to search for
    pub age: String,

    /// Gender group to search for
    pub gender: String,

    /// WSO region to search for
    pub wso: String,
}

pub fn run(_args: WsoRecordsArgs, _convex_url: &str) {
    let _carolina_sheet_id = "1rKFzpkLCT-FE2SzM0qpUOoZ788YHl7dg";
    let _ref_spreadsheet_id = "1ZI9TOZ8Ql-ACxNIcytsPXrWfetZFXyjg";
    let _ref_gid = "1911965444";

    // youth, junior, senior, masters
    let _tabs_id: [&str; 4] = ["1785893123", "1157313505", "2109027801", "448005775"];
}
