use clap::Parser;

use crate::types::records::Record;

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

pub fn run(_args: WsoRecordsArgs, _convex_url: &str) {}
